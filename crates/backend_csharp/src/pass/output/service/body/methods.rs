//! Renders all service method variants per service.
//!
//! For each service method, renders a base forwarding method and any overloads
//! discovered via the central `fns::overload::all` registry. All methods simply
//! forward to `Interop.function_name` — C# overload resolution picks the right
//! variant based on argument types.
//!
//! Argument types are taken directly from the Function objects in `fns::all`,
//! which already have the correct overloaded types (ref, delegate signature, etc.).

use crate::lang::functions::Argument;
use crate::lang::types::{PointerKind, Primitive, TypeKind};
use crate::lang::ServiceId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::{Context, Value};
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
        fn_all: &model::fns::all::Pass,
        type_names: &model::types::names::Pass,
        type_kinds: &model::types::kind::Pass,
        method_names: &model::service::method::names::Pass,
        overload_all: &model::fns::overload::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (service_id, service) in service_map.iter() {
            let mut rendered_methods = Vec::new();

            for &method_fn_id in &service.methods {
                let Some(method_fn) = fn_all.get(method_fn_id) else { continue };
                let Some(method_name) = method_names.get(method_fn_id) else { continue };
                let Some(rval) = type_names.get(method_fn.signature.rval) else { continue };
                let is_void = matches!(type_kinds.get(method_fn.signature.rval), Some(TypeKind::Primitive(Primitive::Void)));

                // Base method
                let base_args = build_args(&method_fn.signature.arguments[1..], type_names, type_kinds);
                rendered_methods.push(render(templates, rval, is_void, method_name, &method_fn.name, &base_args)?);

                // Overloads from the central registry
                if let Some(overload_ids) = overload_all.overloads_for(method_fn_id) {
                    for &overload_id in overload_ids {
                        let Some(overload_fn) = fn_all.get(overload_id) else { continue };
                        let overload_args = build_args(&overload_fn.signature.arguments[1..], type_names, type_kinds);
                        rendered_methods.push(render(templates, rval, is_void, method_name, &overload_fn.name, &overload_args)?);
                    }
                }
            }

            self.body_methods.insert(*service_id, rendered_methods);
        }

        Ok(())
    }

    pub fn get(&self, service_id: ServiceId) -> Option<&[String]> {
        self.body_methods.get(&service_id).map(|v| v.as_slice())
    }
}

fn build_args(args: &[Argument], type_names: &model::types::names::Pass, type_kinds: &model::types::kind::Pass) -> Vec<HashMap<&'static str, Value>> {
    args.iter()
        .filter_map(|arg| {
            let ty_name = type_names.get(arg.ty)?;
            let is_ref = matches!(type_kinds.get(arg.ty), Some(TypeKind::Pointer(p)) if p.kind == PointerKind::ByRef);
            Some(make_arg(&arg.name, ty_name, is_ref))
        })
        .collect()
}

fn make_arg(name: &str, ty: &str, is_ref: bool) -> HashMap<&'static str, Value> {
    let mut m = HashMap::new();
    m.insert("name", Value::String(name.to_string()));
    m.insert("ty", Value::String(ty.to_string()));
    m.insert("is_ref", Value::Bool(is_ref));
    m
}

fn render(
    templates: &interoptopus_backends::template::TemplateEngine,
    rval: &str,
    is_void: bool,
    method_name: &str,
    interop_name: &str,
    args: &[HashMap<&str, Value>],
) -> Result<String, crate::Error> {
    let mut context = Context::new();
    context.insert("rval", rval);
    context.insert("is_void", &is_void);
    context.insert("method_name", &method_name);
    context.insert("interop_name", &interop_name);
    context.insert("args", args);
    Ok(templates.render("service/body_methods.cs", &context)?)
}
