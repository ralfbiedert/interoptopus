//! Renders plain service method wrappers using original types (no overloads).
//!
//! Each method simply forwards to the interop function, passing `_context`
//! as the first argument. This is the fallback when no overloads apply.

use crate::lang::types::{Primitive, TypeKind};
use crate::lang::FunctionId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::{Context, Value};
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
        type_kinds: &model::types::kind::Pass,
        method_names: &model::service::method::names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (_service_id, service) in service_map.iter() {
            for &method_fn_id in &service.methods {
                if self.rendered.contains_key(&method_fn_id) {
                    continue;
                }

                let Some(method_fn) = fn_map.get(method_fn_id) else { continue };
                let Some(rval) = type_names.name(method_fn.signature.rval) else { continue };
                let is_void = matches!(type_kinds.get(method_fn.signature.rval), Some(TypeKind::Primitive(Primitive::Void)));

                // Skip the first argument (instance pointer)
                let mut args: Vec<HashMap<&str, Value>> = Vec::new();
                for arg in method_fn.signature.arguments.iter().skip(1) {
                    let Some(arg_ty) = type_names.name(arg.ty) else { continue };
                    let mut m = HashMap::new();
                    m.insert("name", Value::String(arg.name.clone()));
                    m.insert("ty", Value::String(arg_ty.clone()));
                    m.insert("is_ref", Value::Bool(false));
                    args.push(m);
                }

                let Some(method_name) = method_names.get(method_fn_id) else { continue };

                let mut context = Context::new();
                context.insert("rval", rval);
                context.insert("is_void", &is_void);
                context.insert("method_name", &method_name);
                context.insert("interop_name", &method_fn.name);
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
