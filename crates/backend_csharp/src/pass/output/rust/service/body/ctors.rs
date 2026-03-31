//! Renders constructor methods for each service using the `service/body_ctors.cs` template.
//!
//! Async constructors (those whose native function has an `AsyncCallback` as the last
//! argument) are rendered via `service/body_ctors_async.cs` instead, using the async
//! overload from the central overload registry.

use crate::lang::ServiceId;
use crate::lang::functions::FunctionKind;
use crate::lang::functions::overload::OverloadKind;
use crate::pass::{OutputResult, PassInfo, format_docs, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_ctors: HashMap<ServiceId, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_ctors: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        services: &model::common::service::all::Pass,
        fns: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        method_names: &model::rust::service::method::names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (service_id, service) in services.iter() {
            let Some(name) = types.get(service.ty).map(|t| &t.name) else { continue };

            let mut rendered_ctors = Vec::new();

            for ctor_fn_id in &service.sources.ctors {
                let Some(ctor_fn) = fns.get(*ctor_fn_id) else { continue };
                let Some(method_name) = method_names.get(*ctor_fn_id) else { continue };

                // Check if this constructor has an async overload
                let async_overload = fns.overloads_for(*ctor_fn_id).find_map(|(overload_id, func)| {
                    if let FunctionKind::Overload(o) = &func.kind
                        && let OverloadKind::Async(_) = &o.kind
                    {
                        return Some(*overload_id);
                    }
                    None
                });

                // Check if this constructor has a body overload (service arg transforms)
                let body_overload = fns.overloads_for(*ctor_fn_id).find_map(|(overload_id, func)| {
                    if let FunctionKind::Overload(o) = &func.kind
                        && matches!(&o.kind, OverloadKind::Body(_))
                    {
                        return Some(*overload_id);
                    }
                    None
                });

                let docs = format_docs(&ctor_fn.docs);

                if let Some(overload_id) = async_overload {
                    let Some(overload_fn) = fns.get(overload_id) else { continue };

                    let args = build_args(&overload_fn.signature.arguments, types);

                    let mut context = Context::new();
                    context.insert("name", name);
                    context.insert("method_name", method_name);
                    context.insert("interop_name", &overload_fn.name);
                    context.insert("args", &args);
                    context.insert("docs", &docs);
                    context.insert("visibility", "public");

                    let rendered = templates.render("rust/service/body_ctors_async.cs", &context)?;
                    rendered_ctors.push(rendered);
                } else if let Some(overload_id) = body_overload {
                    let Some(overload_fn) = fns.get(overload_id) else { continue };

                    let args = build_args(&overload_fn.signature.arguments, types);

                    let mut context = Context::new();
                    context.insert("name", name);
                    context.insert("method_name", method_name);
                    context.insert("interop_name", &overload_fn.name);
                    context.insert("args", &args);
                    context.insert("docs", &docs);
                    context.insert("visibility", "public");

                    let rendered = templates.render("rust/service/body_ctors.cs", &context)?;
                    rendered_ctors.push(rendered);
                } else {
                    // Sync constructor with no overload — render with original args
                    let args = build_args(&ctor_fn.signature.arguments, types);

                    let mut context = Context::new();
                    context.insert("name", name);
                    context.insert("method_name", method_name);
                    context.insert("interop_name", &ctor_fn.name);
                    context.insert("args", &args);
                    context.insert("docs", &docs);
                    context.insert("visibility", "public");

                    let rendered = templates.render("rust/service/body_ctors.cs", &context)?;
                    rendered_ctors.push(rendered);
                }
            }

            self.body_ctors.insert(*service_id, rendered_ctors);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, service_id: ServiceId) -> Option<&[String]> {
        self.body_ctors.get(&service_id).map(std::vec::Vec::as_slice)
    }
}

fn build_args(args: &[crate::lang::functions::Argument], types: &model::common::types::all::Pass) -> Vec<HashMap<&'static str, String>> {
    args.iter()
        .filter_map(|arg| {
            let arg_type = types.get(arg.ty)?;
            let decorated = match &arg_type.decorators.param {
                Some(d) => format!("{d} {}", arg_type.name),
                None => arg_type.name.clone(),
            };
            let mut m = HashMap::new();
            m.insert("name", arg.name.clone());
            m.insert("ty", decorated);
            if arg.ty == crate::lang::types::csharp::CANCELLATION_TOKEN {
                m.insert("has_default", "true".to_string());
                m.insert("default_value", "default".to_string());
            } else {
                m.insert("has_default", "false".to_string());
            }
            Some(m)
        })
        .collect()
}
