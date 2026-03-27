//! Renders `[UnmanagedCallersOnly]` trampoline methods for service constructors,
//! methods, and destructors.
//!
//! Service ctors return `TypeName.Unmanaged` via `.IntoUnmanaged()`.
//! Methods dereference the `*const ServiceHandle` self pointer via
//! `Marshal.ReadIntPtr(self)` to recover the `GCHandle`.
//! Destructors receive the `ServiceHandle` by value and free it.
//!
//! Method names and type names are resolved from the service interface model pass.

use crate::lang::FunctionId;
use crate::lang::ServiceId;
use crate::lang::plugin::TrampolineKind;
use crate::lang::service::Service;
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

    #[allow(clippy::too_many_lines)]
    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        trampoline_model: &model::dotnet::trampoline::Pass,
        service_interfaces: &model::dotnet::interface::service::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        services: &model::common::service::all::Pass,
        unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
        unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();
        let svc_lookup: HashMap<ServiceId, &Service> = services.iter().map(|(&id, svc)| (id, svc)).collect();

        // Build FunctionId → C# method name lookup from all service interface models.
        let method_names: HashMap<FunctionId, &str> = service_interfaces
            .interfaces()
            .iter()
            .flat_map(|iface| iface.methods.iter().map(|m| (m.base, m.name.as_str())))
            .collect();

        // Build FunctionId → Result type name lookup for methods with unwrapped Try<T> returns.
        let result_wraps: HashMap<FunctionId, &str> = service_interfaces
            .interfaces()
            .iter()
            .flat_map(|iface| {
                iface.methods.iter().filter_map(|m| {
                    let result_id = m.unwrapped_result_id?;
                    let name = types.get(result_id).map(|t| t.name.as_str())?;
                    Some((m.base, name))
                })
            })
            .collect();

        for entry in trampoline_model.entries() {
            let Some(func) = fns_all.get(entry.fn_id) else { continue };
            let ffi_name = &func.name;
            let result_wrap_type = result_wraps.get(&entry.fn_id).copied().unwrap_or("");

            let rendered = match &entry.kind {
                TrampolineKind::ServiceCtor { service_id } => {
                    let Some(svc) = svc_lookup.get(service_id) else { continue };
                    let type_name = types.get(svc.ty).map_or("", |t| t.name.as_str());
                    let Some(&method_name) = method_names.get(&entry.fn_id) else { continue };
                    let is_async = async_callback_inner(func, types).is_some();

                    if is_async {
                        // Async ctor: strip callback from forwarded args, use continuation.
                        let (args, forward) = unmanaged_args_except_last(func, unmanaged_names, unmanaged_conversion);
                        let cb_inner = async_callback_inner(func, types);
                        let to_unmanaged = cb_inner.map_or("IntoUnmanaged", |id| unmanaged_conversion.to_unmanaged_name(id));
                        let continuation = format!("ContinueWith(t => cb.UnsafeComplete(t.Result.{to_unmanaged}()))");

                        let mut ctx = Context::new();
                        ctx.insert("ffi_name", ffi_name);
                        ctx.insert("args", &args);
                        ctx.insert("type_name", type_name);
                        ctx.insert("method_name", method_name);
                        ctx.insert("forward", &forward);
                        ctx.insert("continuation", &continuation);
                        ctx.insert("result_wrap_type", result_wrap_type);
                        templates.render("dotnet/interop/service_ctor_async.cs", &ctx)?
                    } else if resolve_ptr_to_service_name(func.signature.rval, types).is_some() {
                        // Sync ctor returning bare ServiceHandle → nint.
                        let (args, forward) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);

                        let mut ctx = Context::new();
                        ctx.insert("ffi_name", ffi_name);
                        ctx.insert("args", &args);
                        ctx.insert("type_name", type_name);
                        ctx.insert("method_name", method_name);
                        ctx.insert("forward", &forward);
                        templates.render("dotnet/interop/service_ctor.cs", &ctx)?
                    } else {
                        // Sync ctor returning Result-wrapped handle (e.g., Try<Self>).
                        let (args, forward) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);
                        let rval_type_name = types.get(func.signature.rval).map_or("void", |t| t.name.as_str());
                        let rval_type = rval_unmanaged_name(func, rval_type_name, unmanaged_names);
                        let rval_suffix = unmanaged_conversion.to_unmanaged_suffix(func.signature.rval);

                        let mut ctx = Context::new();
                        ctx.insert("ffi_name", ffi_name);
                        ctx.insert("args", &args);
                        ctx.insert("rval_type", &rval_type);
                        ctx.insert("type_name", type_name);
                        ctx.insert("method_name", method_name);
                        ctx.insert("forward", &forward);
                        ctx.insert("rval_suffix", &rval_suffix);
                        ctx.insert("result_wrap_type", result_wrap_type);
                        templates.render("dotnet/interop/service_ctor_result.cs", &ctx)?
                    }
                }
                TrampolineKind::ServiceMethod { service_id } => {
                    let Some(svc) = svc_lookup.get(service_id) else { continue };
                    let type_name = types.get(svc.ty).map_or("", |t| t.name.as_str());
                    let Some(&method_name) = method_names.get(&entry.fn_id) else { continue };
                    let rval_type = types.get(func.signature.rval).map_or("void", |t| t.name.as_str());
                    let rval_is_service = resolve_ptr_to_service_name(func.signature.rval, types).is_some();
                    let async_inner = async_callback_inner(func, types);

                    if let Some(inner_id) = async_inner {
                        let (args, forward) = service_aware_args_except_last(func, types, unmanaged_names, unmanaged_conversion);
                        let self_args = if args.is_empty() {
                            "nint self".to_string()
                        } else {
                            format!("IntPtr self, {args}")
                        };

                        let continuation = if let Some(svc_name) = resolve_ptr_to_service_name(inner_id, types) {
                            "ContinueWith(t => cb.UnsafeComplete(t.Result.IntoUnmanaged()))".to_string()
                        } else {
                            async_continuation(inner_id, types, unmanaged_conversion)
                        };

                        let mut ctx = Context::new();
                        ctx.insert("ffi_name", ffi_name);
                        ctx.insert("args", &self_args);
                        ctx.insert("type_name", type_name);
                        ctx.insert("method_name", method_name);
                        ctx.insert("forward", &forward);
                        ctx.insert("continuation", &continuation);
                        ctx.insert("result_wrap_type", result_wrap_type);
                        templates.render("dotnet/interop/service_method_async.cs", &ctx)?
                    } else if rval_is_service {
                        let ret_svc_name = resolve_ptr_to_service_name(func.signature.rval, types).unwrap();
                        let (args, forward) = service_aware_args(func, types, unmanaged_names, unmanaged_conversion);
                        let self_args = if args.is_empty() {
                            "nint self".to_string()
                        } else {
                            format!("IntPtr self, {args}")
                        };
                        let rval_type = format!("{ret_svc_name}.Unmanaged");

                        let mut ctx = Context::new();
                        ctx.insert("ffi_name", ffi_name);
                        ctx.insert("rval_type", &rval_type);
                        ctx.insert("args", &self_args);
                        ctx.insert("type_name", type_name);
                        ctx.insert("method_name", method_name);
                        ctx.insert("forward", &forward);
                        templates.render("dotnet/interop/service_method_returns_service.cs", &ctx)?
                    } else {
                        let (args, forward) = service_aware_args(func, types, unmanaged_names, unmanaged_conversion);
                        let rval_unmanaged = rval_unmanaged_name(func, rval_type, unmanaged_names);
                        let rval_suffix = unmanaged_conversion.to_unmanaged_suffix(func.signature.rval);
                        let is_void = rval_type == "void";
                        let self_args = if args.is_empty() {
                            "nint self".to_string()
                        } else {
                            format!("IntPtr self, {args}")
                        };

                        let mut ctx = Context::new();
                        ctx.insert("ffi_name", ffi_name);
                        ctx.insert("rval_type", rval_unmanaged);
                        ctx.insert("args", &self_args);
                        ctx.insert("type_name", type_name);
                        ctx.insert("method_name", method_name);
                        ctx.insert("forward", &forward);
                        ctx.insert("rval_suffix", &rval_suffix);
                        ctx.insert("is_void", &is_void);
                        ctx.insert("result_wrap_type", result_wrap_type);
                        templates.render("dotnet/interop/service_method_sync.cs", &ctx)?
                    }
                }
                TrampolineKind::ServiceDestructor { service_id } => {
                    let Some(svc) = svc_lookup.get(service_id) else { continue };
                    let type_name = types.get(svc.ty).map_or("", |t| t.name.as_str());
                    // Destructor receives ServiceHandle by value (one IntPtr).
                    let args = format!("{type_name}.Unmanaged self");
                    let self_expr = "self._handle";

                    let mut ctx = Context::new();
                    ctx.insert("ffi_name", ffi_name);
                    ctx.insert("args", &args);
                    ctx.insert("self_expr", self_expr);
                    templates.render("dotnet/interop/service_destructor.cs", &ctx)?
                }
                TrampolineKind::Raw => continue,
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

/// Returns the managed service class name if `type_id` is a double-pointer to service
/// (i.e., `*const *const Service` — the FFI form of `&Service`).
fn resolve_double_ptr_to_service_name(type_id: crate::lang::TypeId, types: &model::common::types::all::Pass) -> Option<String> {
    let ty = types.get(type_id)?;
    if let TypeKind::Pointer(outer) = &ty.kind {
        let inner = types.get(outer.target)?;
        if let TypeKind::Pointer(p) = &inner.kind {
            let target = types.get(p.target)?;
            if matches!(&target.kind, TypeKind::Service) {
                return Some(target.name.clone());
            }
        }
    }
    None
}

/// Like `unmanaged_args` but handles pointer-to-service params by unwrapping `GCHandle`,
/// and double-pointer-to-service params (ref params) by dereferencing first.
fn service_aware_args(
    func: &crate::lang::functions::Function,
    types: &model::common::types::all::Pass,
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
        .map(|a| {
            if let Some(svc_name) = resolve_ptr_to_service_name(a.ty, types) {
                // Owned service param — ServiceHandle by value, unwrap GCHandle directly.
                format!("({svc_name})GCHandle.FromIntPtr({}).Target!", a.name)
            } else if let Some(svc_name) = resolve_double_ptr_to_service_name(a.ty, types) {
                // Ref service param — pointer-to-ServiceHandle, dereference then unwrap.
                format!("({svc_name})GCHandle.FromIntPtr({}).Target!", a.name)
            } else {
                format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty))
            }
        })
        .collect();

    (args.join(", "), forward.join(", "))
}

/// Like `unmanaged_args_except_last` but handles service params.
fn service_aware_args_except_last(
    func: &crate::lang::functions::Function,
    types: &model::common::types::all::Pass,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let n = func.signature.arguments.len().saturating_sub(1);

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
        .take(n)
        .map(|a| {
            if let Some(svc_name) = resolve_ptr_to_service_name(a.ty, types) {
                format!("({svc_name})GCHandle.FromIntPtr({}).Target!", a.name)
            } else if let Some(svc_name) = resolve_double_ptr_to_service_name(a.ty, types) {
                format!("({svc_name})GCHandle.FromIntPtr({}).Target!", a.name)
            } else {
                format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty))
            }
        })
        .collect();

    (args.join(", "), forward.join(", "))
}
