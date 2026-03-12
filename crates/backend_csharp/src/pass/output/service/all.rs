//! Renders service classes through the `service/all.cs` template, grouped per output file.

use crate::output::{Output, OutputKind};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    services: HashMap<Output, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, services: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        service_map: &model::service::map::Pass,
        fn_map: &model::fns::all::Pass,
        types: &model::types::all::Pass,
        body_ctors: &output::service::body::ctors::Pass,
        body_methods: &output::service::body::methods::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut rendered_services = Vec::new();

            for (service_id, service) in service_map.iter() {
                let Some(name) = types.get(service.ty).map(|t| &t.name) else { continue };
                let Some(dtor_fn) = fn_map.get(service.destructor) else { continue };
                let ctors = body_ctors.get(*service_id).unwrap_or_default();
                let methods = body_methods.get(*service_id).unwrap_or_default();

                let mut context = Context::new();
                context.insert("name", name);
                context.insert("dtor", &dtor_fn.name);
                context.insert("ctors", &ctors);
                context.insert("methods", &methods);

                let rendered = templates.render("service/all.cs", &context)?;
                rendered_services.push(rendered);
            }

            rendered_services.sort();

            self.services.insert(file.clone(), rendered_services);
        }

        Ok(())
    }

    pub fn services_for(&self, output: &Output) -> Option<&[String]> {
        self.services.get(output).map(|s| s.as_slice())
    }
}
