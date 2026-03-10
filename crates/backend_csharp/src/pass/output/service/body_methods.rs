//! Renders method wrappers for each service using the `service/body_methods.cs` template.

use crate::lang::ServiceId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_methods: HashMap<ServiceId, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_methods: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        service_map: &model::service::map::Pass,
        fn_map: &model::fns::rust::Pass,
        type_names: &model::types::names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (service_id, service) in service_map.iter() {
            let mut rendered_methods = Vec::new();

            for method_fn_id in &service.methods {
                let Some(method_fn) = fn_map.get(*method_fn_id) else { continue };

                let Some(rval) = type_names.name(method_fn.signature.rval) else { continue };

                // Skip the first argument (instance pointer) — it's passed as _context
                let mut args: Vec<HashMap<&str, &str>> = Vec::new();
                for arg in method_fn.signature.arguments.iter().skip(1) {
                    let Some(arg_ty) = type_names.name(arg.ty) else { continue };
                    let mut m = HashMap::new();
                    m.insert("name", arg.name.as_str());
                    m.insert("ty", arg_ty.as_str());
                    args.push(m);
                }

                let method_name = method_name_from_interop(&method_fn.name);

                let mut context = Context::new();
                context.insert("rval", rval);
                context.insert("method_name", &method_name);
                context.insert("interop_name", &method_fn.name);
                context.insert("args", &args);

                let rendered = templates.render("service/body_methods.cs", &context)?;
                rendered_methods.push(rendered);
            }

            self.body_methods.insert(*service_id, rendered_methods);
        }

        Ok(())
    }

    pub fn get(&self, service_id: ServiceId) -> Option<&[String]> {
        self.body_methods.get(&service_id).map(|v| v.as_slice())
    }
}

/// Derives a C# method name from the interop function name.
///
/// E.g., `service_basic_return_default_value` → `ReturnDefaultValue`.
fn method_name_from_interop(interop_name: &str) -> String {
    // The interop name is `{service_snake}_{method_snake}`.
    // We take the last `_`-separated segment and PascalCase it.
    // TODO: This is a simplistic heuristic; may need refinement for multi-word method names.
    let method = interop_name.rsplit('_').next().unwrap_or(interop_name);
    let mut chars = method.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => interop_name.to_string(),
    }
}
