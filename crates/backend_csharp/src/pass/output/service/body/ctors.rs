//! Renders constructor methods for each service using the `service/body_ctors.cs` template.

use crate::lang::ServiceId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::casing::last_segment_to_pascal;
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_ctors: HashMap<ServiceId, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_ctors: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        service_map: &model::service::map::Pass,
        fn_map: &model::fns::all::Pass,
        type_names: &model::types::names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (service_id, service) in service_map.iter() {
            let Some(name) = type_names.name(service.ty) else { continue };

            let mut rendered_ctors = Vec::new();

            for ctor_fn_id in &service.ctors {
                let Some(ctor_fn) = fn_map.get(*ctor_fn_id) else { continue };

                let mut args: Vec<HashMap<&str, &str>> = Vec::new();

                for arg in &ctor_fn.signature.arguments {
                    let Some(arg_ty) = type_names.name(arg.ty) else { continue };
                    let mut m = HashMap::new();
                    m.insert("name", arg.name.as_str());
                    m.insert("ty", arg_ty.as_str());
                    args.push(m);
                }

                let method_name = last_segment_to_pascal(&ctor_fn.name);

                let mut context = Context::new();
                context.insert("name", name);
                context.insert("method_name", &method_name);
                context.insert("interop_name", &ctor_fn.name);
                context.insert("args", &args);

                let rendered = templates.render("service/body_ctors.cs", &context)?;
                rendered_ctors.push(rendered);
            }

            self.body_ctors.insert(*service_id, rendered_ctors);
        }

        Ok(())
    }

    pub fn get(&self, service_id: ServiceId) -> Option<&[String]> {
        self.body_ctors.get(&service_id).map(|v| v.as_slice())
    }
}
