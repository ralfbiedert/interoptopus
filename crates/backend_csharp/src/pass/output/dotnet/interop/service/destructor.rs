//! Renders `[UnmanagedCallersOnly]` trampoline methods for service destructors.
//!
//! Destructors receive the `ServiceHandle` by value (one `IntPtr`) and free
//! the underlying `GCHandle`.

use crate::lang::FunctionId;
use crate::lang::ServiceId;
use crate::lang::plugin::TrampolineKind;
use crate::lang::service::Service;
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
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        services: &model::common::service::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();
        let svc_lookup: HashMap<ServiceId, &Service> = services.iter().map(|(&id, svc)| (id, svc)).collect();

        for entry in trampoline_model.entries() {
            let TrampolineKind::ServiceDestructor { service_id } = &entry.kind else { continue };
            let Some(func) = fns_all.get(entry.fn_id) else { continue };
            let Some(svc) = svc_lookup.get(service_id) else { continue };
            let type_name = types.get(svc.ty).map_or("", |t| t.name.as_str());
            let ffi_name = &func.name;

            // Destructor receives ServiceHandle by value (one IntPtr).
            let args = format!("{type_name}.Unmanaged self");
            let self_expr = "self._handle";

            let mut ctx = Context::new();
            ctx.insert("ffi_name", ffi_name);
            ctx.insert("args", &args);
            ctx.insert("self_expr", self_expr);
            let rendered = templates.render("dotnet/interop/service_destructor.cs", &ctx)?;

            self.methods.insert(entry.fn_id, rendered.trim_end().to_string());
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, fn_id: FunctionId) -> Option<&str> {
        self.methods.get(&fn_id).map(String::as_str)
    }
}
