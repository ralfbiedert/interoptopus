//! Renders `[UnmanagedCallersOnly]` trampoline methods for service constructors,
//! methods, and destructors.
//!
//! Service ctors allocate a `GCHandle`, methods cast `self` back to the interface,
//! and destructors free the handle. Async service methods use `.ContinueWith(...)`
//! to complete the `AsyncCallback`.
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

        for entry in trampoline_model.entries() {
            let Some(func) = fns_all.get(entry.fn_id) else { continue };
            let ffi_name = &func.name;

            let rendered = match &entry.kind {
                TrampolineKind::ServiceCtor { service_id } => {
                    let Some(svc) = svc_lookup.get(service_id) else { continue };
                    let type_name = types.get(svc.ty).map_or("", |t| t.name.as_str());
                    let Some(&method_name) = method_names.get(&entry.fn_id) else { continue };
                    let (args, forward) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);

                    let mut ctx = Context::new();
                    ctx.insert("ffi_name", ffi_name);
                    ctx.insert("args", &args);
                    ctx.insert("type_name", type_name);
                    ctx.insert("method_name", method_name);
                    ctx.insert("forward", &forward);
                    templates.render("dotnet/interop/service_ctor.cs", &ctx)?
                }
                TrampolineKind::ServiceMethod { service_id } => {
                    let Some(svc) = svc_lookup.get(service_id) else { continue };
                    let type_name = types.get(svc.ty).map_or("", |t| t.name.as_str());
                    let Some(&method_name) = method_names.get(&entry.fn_id) else { continue };
                    let rval_type = types.get(func.signature.rval).map_or("void", |t| t.name.as_str());
                    let rval_is_service = matches!(types.get(func.signature.rval).map(|t| &t.kind), Some(TypeKind::Service));
                    let async_inner = async_callback_inner(func, types);

                    if let Some(inner_id) = async_inner {
                        let (args, forward) = service_aware_args_except_last(func, types, unmanaged_names, unmanaged_conversion);
                        let self_args = if args.is_empty() { "nint self".to_string() } else { format!("nint self, {args}") };
                        let continuation = if matches!(types.get(inner_id).map(|t| &t.kind), Some(TypeKind::Service)) {
                            "ContinueWith(t => { var h = GCHandle.Alloc(t.Result); cb.UnsafeComplete(GCHandle.ToIntPtr(h)); })".to_string()
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
                        templates.render("dotnet/interop/service_method_async.cs", &ctx)?
                    } else if rval_is_service {
                        // Return type is a service — wrap result in GCHandle (same pattern as ctors)
                        let (args, forward) = service_aware_args(func, types, unmanaged_names, unmanaged_conversion);
                        let self_args = if args.is_empty() { "nint self".to_string() } else { format!("nint self, {args}") };

                        let mut ctx = Context::new();
                        ctx.insert("ffi_name", ffi_name);
                        ctx.insert("args", &self_args);
                        ctx.insert("type_name", type_name);
                        ctx.insert("method_name", method_name);
                        ctx.insert("forward", &forward);
                        // Reuse the ctor template which does GCHandle.Alloc + ToIntPtr
                        templates.render("dotnet/interop/service_method_returns_service.cs", &ctx)?
                    } else {
                        let (args, forward) = service_aware_args(func, types, unmanaged_names, unmanaged_conversion);
                        let rval_unmanaged = rval_unmanaged_name(func, rval_type, unmanaged_names);
                        let rval_suffix = unmanaged_conversion.to_unmanaged_suffix(func.signature.rval);
                        let is_void = rval_type == "void";
                        let self_args = if args.is_empty() { "nint self".to_string() } else { format!("nint self, {args}") };

                        let mut ctx = Context::new();
                        ctx.insert("ffi_name", ffi_name);
                        ctx.insert("rval_type", rval_unmanaged);
                        ctx.insert("args", &self_args);
                        ctx.insert("type_name", type_name);
                        ctx.insert("method_name", method_name);
                        ctx.insert("forward", &forward);
                        ctx.insert("rval_suffix", &rval_suffix);
                        ctx.insert("is_void", &is_void);
                        templates.render("dotnet/interop/service_method_sync.cs", &ctx)?
                    }
                }
                TrampolineKind::ServiceDestructor { .. } => {
                    let mut ctx = Context::new();
                    ctx.insert("ffi_name", ffi_name);
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

/// Like `unmanaged_args` but handles service type params by unwrapping GCHandle.
fn service_aware_args(
    func: &crate::lang::functions::Function,
    types: &model::common::types::all::Pass,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let args: Vec<String> = func.signature.arguments.iter().filter_map(|arg| {
        let ty_name = unmanaged_names.name(arg.ty)?;
        Some(format!("{ty_name} {}", arg.name))
    }).collect();

    let forward: Vec<String> = func.signature.arguments.iter().map(|a| {
        if matches!(types.get(a.ty).map(|t| &t.kind), Some(TypeKind::Service)) {
            let ty_name = types.get(a.ty).map_or("object", |t| t.name.as_str());
            format!("({ty_name})GCHandle.FromIntPtr({}).Target!", a.name)
        } else {
            format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty))
        }
    }).collect();

    (args.join(", "), forward.join(", "))
}

/// Like `unmanaged_args_except_last` but handles service type params by unwrapping GCHandle.
fn service_aware_args_except_last(
    func: &crate::lang::functions::Function,
    types: &model::common::types::all::Pass,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let n = func.signature.arguments.len().saturating_sub(1);

    let args: Vec<String> = func.signature.arguments.iter().filter_map(|arg| {
        let ty_name = unmanaged_names.name(arg.ty)?;
        Some(format!("{ty_name} {}", arg.name))
    }).collect();

    let forward: Vec<String> = func.signature.arguments.iter().take(n).map(|a| {
        if matches!(types.get(a.ty).map(|t| &t.kind), Some(TypeKind::Service)) {
            let ty_name = types.get(a.ty).map_or("object", |t| t.name.as_str());
            format!("({ty_name})GCHandle.FromIntPtr({}).Target!", a.name)
        } else {
            format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty))
        }
    }).collect();

    (args.join(", "), forward.join(", "))
}
