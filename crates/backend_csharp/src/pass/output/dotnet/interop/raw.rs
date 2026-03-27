//! Renders `[UnmanagedCallersOnly]` trampoline methods for raw (non-service) functions.
//!
//! Each `TrampolineKind::Raw` entry produces one method inside `public static class Interop`.
//! Sync functions forward to `Plugin.Method(args…)`, while async functions attach a
//! `.ContinueWith(...)` continuation that completes the `AsyncCallback`.
//!
//! Functions returning service types use `.IntoUnmanaged()` to convert to the FFI form.
//!
//! Method names are resolved from the plugin interface model pass.

use crate::lang::FunctionId;
use crate::lang::TypeId;
use crate::lang::plugin::TrampolineKind;
use crate::lang::types::kind::TypeKind;
use crate::pass::output::dotnet::interop::{async_callback_inner, async_continuation, rval_unmanaged_name, unmanaged_args, unmanaged_args_except_last};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    methods: HashMap<FunctionId, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, methods: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        trampoline_model: &model::dotnet::trampoline::Pass,
        plugin_interface: &model::dotnet::interface::plugin::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
        unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        // Build FunctionId → C# method name lookup from the plugin interface model.
        let method_names: HashMap<FunctionId, &str> = plugin_interface
            .interface()
            .map(|iface| iface.methods.iter().map(|m| (m.base, m.name.as_str())).collect())
            .unwrap_or_default();

        // Build FunctionId → Result type name lookup for methods with unwrapped Try<T> returns.
        let result_wraps: HashMap<FunctionId, &str> = plugin_interface
            .interface()
            .map(|iface| {
                iface
                    .methods
                    .iter()
                    .filter_map(|m| {
                        let result_id = m.unwrapped_result_id?;
                        let name = types.get(result_id).map(|t| t.name.as_str())?;
                        Some((m.base, name))
                    })
                    .collect()
            })
            .unwrap_or_default();

        for entry in trampoline_model.entries() {
            if !matches!(entry.kind, TrampolineKind::Raw) {
                continue;
            }

            let Some(func) = fns_all.get(entry.fn_id) else { continue };
            let Some(&pascal_name) = method_names.get(&entry.fn_id) else { continue };

            let ffi_name = &func.name;
            let rval_type = types.get(func.signature.rval).map_or("void", |t| t.name.as_str());
            let async_inner = async_callback_inner(func, types);
            let rval_is_service = resolve_ptr_to_service_name(func.signature.rval, types).is_some();
            let result_wrap_type = result_wraps.get(&entry.fn_id).copied().unwrap_or("");

            let rendered = if let Some(inner_id) = async_inner {
                let (args, forward) = unmanaged_args_except_last(func, unmanaged_names, unmanaged_conversion);

                // Async service return: use .IntoUnmanaged() in continuation.
                let continuation = if let Some(_svc_name) = resolve_ptr_to_service_name(inner_id, types) {
                    "ContinueWith(t => cb.UnsafeComplete(t.Result.IntoUnmanaged()))".to_string()
                } else {
                    async_continuation(inner_id, types, unmanaged_conversion)
                };

                let mut ctx = Context::new();
                ctx.insert("ffi_name", ffi_name);
                ctx.insert("args", &args);
                ctx.insert("pascal_name", pascal_name);
                ctx.insert("forward", &forward);
                ctx.insert("continuation", &continuation);
                ctx.insert("result_wrap_type", result_wrap_type);
                templates.render("dotnet/interop/raw_async.cs", &ctx)?
            } else if rval_is_service {
                // Sync raw function returning a service.
                let svc_name = resolve_ptr_to_service_name(func.signature.rval, types).unwrap();
                let (args, forward) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);
                let rval_type = format!("{svc_name}.Unmanaged");

                let mut ctx = Context::new();
                ctx.insert("ffi_name", ffi_name);
                ctx.insert("rval_type", &rval_type);
                ctx.insert("args", &args);
                ctx.insert("pascal_name", pascal_name);
                ctx.insert("forward", &forward);
                ctx.insert("rval_suffix", ".IntoUnmanaged()");
                ctx.insert("is_void", &false);
                ctx.insert("result_wrap_type", result_wrap_type);
                templates.render("dotnet/interop/raw_sync.cs", &ctx)?
            } else {
                let (args, forward) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);
                let rval_unmanaged = rval_unmanaged_name(func, rval_type, unmanaged_names);
                let rval_suffix = unmanaged_conversion.to_unmanaged_suffix(func.signature.rval);
                let is_void = rval_type == "void";

                let mut ctx = Context::new();
                ctx.insert("ffi_name", ffi_name);
                ctx.insert("rval_type", rval_unmanaged);
                ctx.insert("args", &args);
                ctx.insert("pascal_name", pascal_name);
                ctx.insert("forward", &forward);
                ctx.insert("rval_suffix", &rval_suffix);
                ctx.insert("is_void", &is_void);
                ctx.insert("result_wrap_type", result_wrap_type);
                templates.render("dotnet/interop/raw_sync.cs", &ctx)?
            };

            self.methods.insert(entry.fn_id, rendered.trim_end().to_string());
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, fn_id: FunctionId) -> Option<&str> {
        self.methods.get(&fn_id).map(String::as_str)
    }
}

/// Returns the managed service class name if `type_id` is a pointer-to-service.
fn resolve_ptr_to_service_name(type_id: crate::lang::TypeId, types: &model::common::types::all::Pass) -> Option<String> {
    let ty = types.get(type_id)?;
    if let TypeKind::Pointer(p) = &ty.kind {
        let target = types.get(p.target)?;
        if matches!(&target.kind, TypeKind::Service) {
            return Some(target.name.clone());
        }
    }
    None
}
