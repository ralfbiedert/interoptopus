//! Renders service classes through the `service/all.cs` template, grouped per output file.

use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    services: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, services: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        services: &model::common::service::all::Pass,
        fns: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        body_ctors: &output::rust::service::body::ctors::Pass,
        body_methods: &output::rust::service::body::methods::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered_services = Vec::new();

            for (service_id, service) in services.iter() {
                if !output_master.type_belongs_to(service.ty, file) {
                    continue;
                }

                let Some(name) = types.get(service.ty).map(|t| &t.name) else { continue };
                let Some(dtor_fn) = fns.get(service.destructor) else { continue };
                let ctors = body_ctors.get(*service_id).unwrap_or_default();
                let methods = body_methods.get(*service_id).unwrap_or_default();

                let mut context = Context::new();
                context.insert("name", name);
                context.insert("dtor", &dtor_fn.name);
                context.insert("ctors", &ctors);
                context.insert("methods", &methods);
                let rendered = templates.render("rust/service/all.cs", &context)?;
                rendered_services.push(rendered);
            }

            rendered_services.sort();

            self.services.insert(file.clone(), rendered_services);
        }

        Ok(())
    }

    #[must_use]
    pub fn services_for(&self, output: &Output) -> Option<&[String]> {
        self.services.get(output).map(std::vec::Vec::as_slice)
    }
}
