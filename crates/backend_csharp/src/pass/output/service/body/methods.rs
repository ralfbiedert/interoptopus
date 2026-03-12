//! Renders all service method variants per service.
//!
//! For each service method, renders a base forwarding method and any overloads
//! discovered via the central `fns::overload::all` registry. All methods simply
//! forward to `Interop.function_name` — C# overload resolution picks the right
//! variant based on argument types.
//!
//! Argument types are taken directly from the Function objects in `fns::all`,
//! which already have the correct overloaded types (ref, delegate signature, etc.).

use crate::lang::ServiceId;
use crate::lang::functions::Argument;
use crate::lang::functions::overload::{OverloadKind, RvalTransform};
use crate::lang::types::kind::{PointerKind, Primitive, TypeKind, TypePattern};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::{Context, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_methods: HashMap<ServiceId, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_methods: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        services: &model::service::all::Pass,
        fns: &model::fns::all::Pass,
        types: &model::types::all::Pass,
        method_names: &model::service::method::names::Pass,
        overload_all: &model::fns::overload::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (service_id, service) in services.iter() {
            let mut rendered_methods = Vec::new();

            for &method_fn_id in &service.methods {
                let Some(method_fn) = fns.get(method_fn_id) else { continue };
                let Some(method_name) = method_names.get(method_fn_id) else { continue };
                let Some(rval) = types.get(method_fn.signature.rval).map(|t| &t.name) else {
                    continue;
                };
                let is_void = matches!(types.get(method_fn.signature.rval).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));

                // Base method (the raw native forwarding method)
                let base_args = build_args(&method_fn.signature.arguments[1..], types);
                rendered_methods.push(render(templates, rval, is_void, method_name, &method_fn.name, &base_args)?);

                // Overloads from the central registry
                if let Some(overload_entries) = overload_all.overloads_for(method_fn_id) {
                    for (overload_id, kind) in overload_entries {
                        let Some(overload_fn) = fns.get(*overload_id) else { continue };

                        if let OverloadKind::Async(transforms) = kind {
                            // Render the async service method: Task<T>
                            if let RvalTransform::AsyncTask(result_ty_id) = transforms.rval {
                                let task_rval = types
                                    .get(result_ty_id)
                                    .map_or_else(|| "Task".to_string(), |t| resolve_task_type_from_result(&t.kind, types));
                                // Skip the self arg (first) for service method args
                                let async_args = build_args(&overload_fn.signature.arguments[1..], types);
                                rendered_methods.push(render_async(templates, &task_rval, method_name, &overload_fn.name, &async_args)?);
                            }
                        } else {
                            let overload_args = build_args(&overload_fn.signature.arguments[1..], types);
                            rendered_methods.push(render(templates, rval, is_void, method_name, &overload_fn.name, &overload_args)?);
                        }
                    }
                }
            }

            self.body_methods.insert(*service_id, rendered_methods);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, service_id: ServiceId) -> Option<&[String]> {
        self.body_methods.get(&service_id).map(std::vec::Vec::as_slice)
    }
}

fn build_args(args: &[Argument], types: &model::types::all::Pass) -> Vec<HashMap<&'static str, Value>> {
    args.iter()
        .filter_map(|arg| {
            let ty_name = types.get(arg.ty).map(|t| &t.name)?;
            let is_ref = matches!(types.get(arg.ty).map(|t| &t.kind), Some(TypeKind::Pointer(p)) if p.kind == PointerKind::ByRef);
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

fn render_async(
    templates: &interoptopus_backends::template::TemplateEngine,
    task_rval: &str,
    method_name: &str,
    interop_name: &str,
    args: &[HashMap<&str, Value>],
) -> Result<String, crate::Error> {
    let mut context = Context::new();
    context.insert("task_rval", task_rval);
    context.insert("method_name", method_name);
    context.insert("interop_name", interop_name);
    context.insert("args", args);
    Ok(templates.render("service/body_methods_async.cs", &context)?)
}

fn resolve_task_type_from_result(result_kind: &TypeKind, types: &model::types::all::Pass) -> String {
    match result_kind {
        TypeKind::TypePattern(TypePattern::Result(ok_ty, _, _)) => {
            let ok_kind = types.get(*ok_ty).map(|t| &t.kind);
            if matches!(ok_kind, Some(TypeKind::Primitive(Primitive::Void))) {
                "Task".to_string()
            } else {
                let ok_name = types.get(*ok_ty).map_or_else(|| "void".to_string(), |t| t.name.clone());
                format!("Task<{ok_name}>")
            }
        }
        _ => "Task".to_string(),
    }
}
