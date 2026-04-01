//! Renders `[UnmanagedCallersOnly]` trampoline methods for synchronous raw (non-service)
//! functions.
//!
//! Handles two variants:
//! - **Service-returning**: the return value is `TypeName.Unmanaged` via `.IntoUnmanaged()`.
//! - **Regular**: standard unmanaged return type with optional conversion suffix.

use crate::lang::FunctionId;
use crate::lang::plugin::TrampolineKind;
use crate::pass::output::dotnet::interop::raw::resolve_ptr_to_service_name;
use crate::pass::output::dotnet::interop::{rval_unmanaged_name, unmanaged_args};
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

        let method_names: HashMap<FunctionId, &str> = plugin_interface
            .interface()
            .map(|iface| iface.methods.iter().map(|m| (m.base, m.name.as_str())).collect())
            .unwrap_or_default();

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

            // Skip async raw functions — handled by async_fn::Pass.
            if crate::pass::output::dotnet::interop::async_callback_inner(func, types).is_some() {
                continue;
            }

            let ffi_name = &func.name;
            let rval_type = types.get(func.signature.rval).map_or("void", |t| t.name.as_str());
            let rval_is_service = resolve_ptr_to_service_name(func.signature.rval, types).is_some();
            let result_wrap_type = result_wraps.get(&entry.fn_id).copied().unwrap_or("");

            let rendered = if rval_is_service {
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
