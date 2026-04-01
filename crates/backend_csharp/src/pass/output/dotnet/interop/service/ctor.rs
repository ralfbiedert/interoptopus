//! Renders `[UnmanagedCallersOnly]` trampoline methods for service constructors.
//!
//! Handles three constructor variants:
//! - **Async ctors**: strip the callback arg from forwarded args and attach a
//!   `.ContinueWith(...)` continuation that completes the `AsyncCallback`.
//! - **Sync bare ctors**: return a `ServiceHandle` directly as `nint`.
//! - **Sync result ctors**: return a `Result`-wrapped handle (e.g. `Try<Self>`).

use crate::lang::FunctionId;
use crate::lang::ServiceId;
use crate::lang::plugin::TrampolineKind;
use crate::lang::service::Service;
use crate::pass::output::dotnet::interop::service::{resolve_ptr_to_service_name, service_aware_args, service_aware_args_except_last};
use crate::pass::output::dotnet::interop::{async_callback_inner, async_continuation, rval_unmanaged_name};
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

        let method_names: HashMap<FunctionId, &str> = service_interfaces
            .interfaces()
            .iter()
            .flat_map(|iface| iface.methods.iter().map(|m| (m.base, m.name.as_str())))
            .collect();

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
            let TrampolineKind::ServiceCtor { service_id } = &entry.kind else { continue };
            let Some(func) = fns_all.get(entry.fn_id) else { continue };
            let Some(svc) = svc_lookup.get(service_id) else { continue };
            let type_name = types.get(svc.ty).map_or("", |t| t.name.as_str());
            let Some(&method_name) = method_names.get(&entry.fn_id) else { continue };
            let ffi_name = &func.name;
            let result_wrap_type = result_wraps.get(&entry.fn_id).copied().unwrap_or("");
            let is_async = async_callback_inner(func, types).is_some();

            let rendered = if is_async {
                // Async ctor: strip callback from forwarded args, use continuation.
                let (args, forward) = service_aware_args_except_last(func, types, unmanaged_names, unmanaged_conversion);
                let cb_inner = async_callback_inner(func, types);

                let continuation = if let Some(inner_id) = cb_inner {
                    if resolve_ptr_to_service_name(inner_id, types).is_some() {
                        "ContinueWith(t => cb.UnsafeComplete(t.Result.IntoUnmanaged()))".to_string()
                    } else {
                        async_continuation(inner_id, types, unmanaged_conversion)
                    }
                } else {
                    "ContinueWith(_ => cb.UnsafeComplete())".to_string()
                };

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
                let (args, forward) = service_aware_args(func, types, unmanaged_names, unmanaged_conversion);

                let mut ctx = Context::new();
                ctx.insert("ffi_name", ffi_name);
                ctx.insert("args", &args);
                ctx.insert("type_name", type_name);
                ctx.insert("method_name", method_name);
                ctx.insert("forward", &forward);
                templates.render("dotnet/interop/service_ctor.cs", &ctx)?
            } else {
                // Sync ctor returning Result-wrapped handle (e.g., Try<Self>).
                let (args, forward) = service_aware_args(func, types, unmanaged_names, unmanaged_conversion);
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
