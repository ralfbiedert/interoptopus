//! Renders service method wrappers that use simple overload signatures (`ref` types).
//!
//! When a method's function has a simple overload (IntPtr replaced by ref), this pass
//! renders the method using the overloaded signature. The method still just forwards
//! to the interop function — no wrapping or disposal needed.

use crate::lang::FunctionId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    rendered: HashMap<FunctionId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, rendered: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        service_map: &model::service::map::Pass,
        fn_map: &model::fns::all::Pass,
        type_names: &model::types::names::Pass,
        method_names: &model::service::method::names::Pass,
        overload_simple: &model::fns::overload::simple::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (_service_id, service) in service_map.iter() {
            for &method_fn_id in &service.methods {
                if self.rendered.contains_key(&method_fn_id) {
                    continue;
                }

                let Some(overload_ids) = overload_simple.overloads_for(method_fn_id) else { continue };
                let Some(&simple_id) = overload_ids.first() else { continue };
                let Some(simple_fn) = fn_map.get(simple_id) else { continue };
                let Some(rval) = type_names.name(simple_fn.signature.rval) else { continue };

                // Skip the first argument (instance pointer)
                let mut args: Vec<HashMap<&str, &str>> = Vec::new();
                for arg in simple_fn.signature.arguments.iter().skip(1) {
                    let Some(arg_ty) = type_names.name(arg.ty) else { continue };
                    let mut m = HashMap::new();
                    m.insert("name", arg.name.as_str());
                    m.insert("ty", arg_ty.as_str());
                    args.push(m);
                }

                let Some(method_name) = method_names.get(method_fn_id) else { continue };

                let mut context = Context::new();
                context.insert("rval", rval);
                context.insert("method_name", &method_name);
                context.insert("interop_name", &simple_fn.name);
                context.insert("args", &args);

                let rendered = templates.render("service/body_methods.cs", &context)?;
                self.rendered.insert(method_fn_id, rendered);
            }
        }

        Ok(())
    }

    pub fn get(&self, fn_id: FunctionId) -> Option<&str> {
        self.rendered.get(&fn_id).map(|s| s.as_str())
    }
}
