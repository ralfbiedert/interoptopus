//! Renders the `public static class Interop` with `[UnmanagedCallersOnly]` trampoline methods.
//!
//! Each trampoline entry produces one exported method. Raw functions forward to
//! `Plugin.Method(args…)`. Service ctors/methods/destructors use `GCHandle` for
//! lifetime management.
//!
//! `Wire<T>` parameters and return values use the Wire type name (e.g. `WireOfString`)
//! in both the interface and the trampoline. The plugin implementor is responsible
//! for calling `.Unwire()` / `.From()` as needed.

use crate::lang::ServiceId;
use crate::lang::plugin::TrampolineKind;
use crate::lang::service::Service;
use crate::lang::types::kind::Primitive;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::lang::TypeId;
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use crate::lang::functions::Function;
use interoptopus_backends::casing::{rust_to_pascal, service_method_name};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    trampolines: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, trampolines: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        trampoline_model: &model::dotnet::trampoline::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        services: &model::common::service::all::Pass,
        unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
        unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
    ) -> OutputResult {
        let svc_lookup: HashMap<ServiceId, &Service> = services.iter().map(|(&id, svc)| (id, svc)).collect();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut methods = Vec::new();

            // The register_trampoline entry point — always emitted so the Rust
            // host can register runtime callbacks (wire alloc/free, etc.).
            methods.push(
                "    [UnmanagedCallersOnly]\n    \
                 public static void register_trampoline(long id, IntPtr fn_ptr) => Trampolines.Register(id, fn_ptr);"
                    .to_string(),
            );

            for entry in trampoline_model.entries() {
                let Some(func) = fns_all.get(entry.fn_id) else { continue };

                let ffi_name = &func.name;
                let rval_type = types.get(func.signature.rval).map(|t| t.name.as_str()).unwrap_or("void");

                let method = match &entry.kind {
                    TrampolineKind::Raw => {
                        let pascal_name = rust_to_pascal(ffi_name);
                        let async_inner = async_callback_inner(func, types);
                        if let Some(inner_id) = async_inner {
                            let (args_str, forward_str) = unmanaged_args_except_last(func, unmanaged_names, unmanaged_conversion);
                            let continuation = async_continuation(inner_id, types, unmanaged_conversion);
                            let inner = format!("_ = Plugin.{pascal_name}({forward_str}).{continuation};");
                            method_block(ffi_name, "void", &args_str, &try_catch_void(&inner))
                        } else {
                            let (args_str, forward_str) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);
                            let rval_unmanaged = rval_unmanaged_name(func, rval_type, unmanaged_names);
                            let call = format!("Plugin.{pascal_name}({forward_str})");
                            let rval_suffix = unmanaged_conversion.to_unmanaged_suffix(func.signature.rval);
                            let inner = format!("{call}{rval_suffix}");
                            let body = if rval_type == "void" {
                                try_catch_void(&format!("{inner};"))
                            } else {
                                try_catch_return(&inner)
                            };
                            method_block(ffi_name, rval_unmanaged, &args_str, &body)
                        }
                    }
                    TrampolineKind::ServiceCtor { service_id } => {
                        let Some(svc) = svc_lookup.get(service_id) else { continue };
                        let type_name = types.get(svc.ty).map(|t| t.name.as_str()).unwrap_or("");
                        let method_name = service_method_name(type_name, ffi_name);
                        let (args_str, forward_str) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);
                        let inner = format!(
                            "var obj = {type_name}.{method_name}({forward_str});\n            \
                             var handle = GCHandle.Alloc(obj);\n            \
                             return GCHandle.ToIntPtr(handle);"
                        );
                        method_block(ffi_name, "nint", &args_str, &try_catch_inner(&inner, true))
                    }
                    TrampolineKind::ServiceMethod { service_id } => {
                        let Some(svc) = svc_lookup.get(service_id) else { continue };
                        let type_name = types.get(svc.ty).map(|t| t.name.as_str()).unwrap_or("");
                        let method_name = service_method_name(type_name, ffi_name);
                        let async_inner = async_callback_inner(func, types);

                        if let Some(inner_id) = async_inner {
                            let (args_str, forward_str) = unmanaged_args_except_last(func, unmanaged_names, unmanaged_conversion);
                            let self_args_str = if args_str.is_empty() {
                                "nint self".to_string()
                            } else {
                                format!("nint self, {args_str}")
                            };
                            let continuation = async_continuation(inner_id, types, unmanaged_conversion);
                            let inner = format!(
                                "var handle = GCHandle.FromIntPtr(self);\n            \
                                 var obj = (I{type_name}<{type_name}>)handle.Target!;\n            \
                                 _ = obj.{method_name}({forward_str}).{continuation};"
                            );
                            method_block(ffi_name, "void", &self_args_str, &try_catch_void(&inner))
                        } else {
                            let (args_str, forward_str) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);
                            let rval_unmanaged = rval_unmanaged_name(func, rval_type, unmanaged_names);
                            let self_args_str = if args_str.is_empty() {
                                "nint self".to_string()
                            } else {
                                format!("nint self, {args_str}")
                            };
                            let call = format!("obj.{method_name}({forward_str})");
                            let rval_suffix = unmanaged_conversion.to_unmanaged_suffix(func.signature.rval);
                            let prelude = format!(
                                "var handle = GCHandle.FromIntPtr(self);\n            \
                                 var obj = (I{type_name}<{type_name}>)handle.Target!;"
                            );
                            let body = if rval_type == "void" {
                                let inner = format!("{prelude}\n            {call};");
                                try_catch_void(&inner)
                            } else {
                                let inner = format!("{prelude}\n            return {call}{rval_suffix};");
                                try_catch_inner(&inner, true)
                            };
                            method_block(ffi_name, rval_unmanaged, &self_args_str, &body)
                        }
                    }
                    TrampolineKind::ServiceDestructor { .. } => {
                        let inner = "var handle = GCHandle.FromIntPtr(self);\n            handle.Free();";
                        method_block(ffi_name, "void", "nint self", &try_catch_void(inner))
                    }
                };

                methods.push(method);
            }

            self.trampolines.insert(file.clone(), methods);
        }

        Ok(())
    }

    #[must_use]
    pub fn trampolines_for(&self, output: &Output) -> Option<&[String]> {
        self.trampolines.get(output).map(Vec::as_slice)
    }
}

/// Returns `(args_str, forward_str)` for a `[UnmanagedCallersOnly]` signature.
///
/// - `args_str`: unmanaged parameter types (e.g. `MyCallback.Unmanaged res`)
/// - `forward_str`: forwarded expressions with to-managed conversions (e.g. `res.IntoManaged()`)
fn unmanaged_args(
    func: &Function,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let args: Vec<String> = func
        .signature
        .arguments
        .iter()
        .filter_map(|arg| {
            let ty_name = unmanaged_names.name(arg.ty)?;
            Some(format!("{ty_name} {}", arg.name))
        })
        .collect();

    let forward: Vec<String> = func
        .signature
        .arguments
        .iter()
        .map(|a| format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty)))
        .collect();

    (args.join(", "), forward.join(", "))
}

/// Returns the unmanaged return type name, or `rval_type` unchanged for `AsIs`/void.
fn rval_unmanaged_name<'a>(
    func: &Function,
    rval_type: &'a str,
    unmanaged_names: &'a output::common::conversion::unmanaged_names::Pass,
) -> &'a str {
    unmanaged_names.name(func.signature.rval).map(String::as_str).unwrap_or(rval_type)
}

/// Returns `(args_str, forward_str)` for an async `[UnmanagedCallersOnly]` signature.
///
/// - `args_str`: all unmanaged parameters including the trailing `AsyncCallback` arg
/// - `forward_str`: only the non-callback args, forwarded with to-managed conversions
fn unmanaged_args_except_last(
    func: &Function,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let n = func.signature.arguments.len().saturating_sub(1);

    // All args in the unmanaged signature (callback stays as-is, it's AsIs/blittable).
    let args: Vec<String> = func
        .signature
        .arguments
        .iter()
        .filter_map(|arg| {
            let ty_name = unmanaged_names.name(arg.ty)?;
            Some(format!("{ty_name} {}", arg.name))
        })
        .collect();

    // Only forward the non-callback args to the managed call.
    let forward: Vec<String> = func
        .signature
        .arguments
        .iter()
        .take(n)
        .map(|a| format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty)))
        .collect();

    (args.join(", "), forward.join(", "))
}

/// If the last argument is `AsyncCallback<T>`, returns the inner `TypeId`.
fn async_callback_inner(func: &Function, types: &model::common::types::all::Pass) -> Option<TypeId> {
    let last = func.signature.arguments.last()?;
    let ty = types.get(last.ty)?;
    match &ty.kind {
        TypeKind::TypePattern(TypePattern::AsyncCallback(inner)) => Some(*inner),
        _ => None,
    }
}

/// Renders a complete `[UnmanagedCallersOnly]` method block.
fn method_block(ffi_name: &str, rval: &str, args: &str, body: &str) -> String {
    format!(
        "    [UnmanagedCallersOnly]\n    \
         public static {rval} {ffi_name}({args})\n    \
         {{\n        \
             {body}\n    \
         }}"
    )
}

/// Wraps `inner` in a try/catch for void methods.
fn try_catch_void(inner: &str) -> String {
    format!(
        "try\n        {{\n            \
             {inner}\n        \
         }}\n        \
         catch (Exception e)\n        \
         {{\n            \
             Trampolines.UncaughtException(e.ToString());\n        \
         }}"
    )
}

/// Wraps `inner` (which already contains `return`) in a try/catch that returns `default` on error.
fn try_catch_return(expr: &str) -> String {
    try_catch_inner(&format!("return {expr};"), true)
}

/// Wraps a multi-statement `inner` in a try/catch. If `has_return` is true the catch returns `default`.
fn try_catch_inner(inner: &str, has_return: bool) -> String {
    let catch_tail = if has_return { "\n            return default;" } else { "" };
    format!(
        "try\n        {{\n            \
             {inner}\n        \
         }}\n        \
         catch (Exception e)\n        \
         {{\n            \
             Trampolines.UncaughtException(e.ToString());{catch_tail}\n        \
         }}"
    )
}

/// Returns the `.ContinueWith(...)` expression that invokes `cb.UnsafeComplete` after the task.
fn async_continuation(inner_id: TypeId, types: &model::common::types::all::Pass, unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass) -> String {
    let is_void = matches!(types.get(inner_id).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    if is_void {
        "ContinueWith(_ => cb.UnsafeComplete())".to_string()
    } else {
        let suffix = unmanaged_conversion.to_unmanaged_suffix(inner_id);
        format!("ContinueWith(t => cb.UnsafeComplete(t.Result{suffix}))")
    }
}
