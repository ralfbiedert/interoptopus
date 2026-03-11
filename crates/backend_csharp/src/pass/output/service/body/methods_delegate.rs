//! Renders service method wrappers with delegate wrapping (body overload transforms).
//!
//! When a method's function has body overload transforms (delegate arguments), this pass
//! renders an overloaded method that accepts bare C# delegates, wraps them into their
//! class, calls the interop function with `_context`, and disposes the wrappers.

use crate::lang::functions::overload::ArgTransform;
use crate::lang::types::{Primitive, TypeKind};
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
        type_kinds: &model::types::kind::Pass,
        method_names: &model::service::method_names::Pass,
        overload_body: &model::fns::overload::body::Pass,
        pointer_overloads: &model::types::overload::pointer::Pass,
        delegate_overloads: &model::types::overload::delegate::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (_service_id, service) in service_map.iter() {
            for &method_fn_id in &service.methods {
                if self.rendered.contains_key(&method_fn_id) {
                    continue;
                }

                let Some(transforms) = overload_body.transforms(method_fn_id) else { continue };
                let Some(method_fn) = fn_map.get(method_fn_id) else { continue };
                let name = &method_fn.name;

                let Some(rval) = type_names.name(method_fn.signature.rval) else { continue };
                let is_void = matches!(type_kinds.get(method_fn.signature.rval), Some(TypeKind::Primitive(Primitive::Void)));

                // Skip the first argument (instance pointer) and its transform
                let mut args: Vec<HashMap<String, String>> = Vec::new();
                for (arg, transform) in method_fn.signature.arguments.iter().zip(&transforms.args).skip(1) {
                    let mut m = HashMap::new();
                    m.insert("name".to_string(), arg.name.clone());

                    match transform {
                        ArgTransform::PassThrough => {
                            let ty_name = type_names
                                .name(arg.ty)
                                .ok_or_else(|| crate::Error::MissingTypeName(format!("arg `{}` of method `{}`", arg.name, name)))?;
                            m.insert("ty".to_string(), ty_name.clone());
                            m.insert("is_ref".to_string(), "false".to_string());
                            m.insert("is_wrap".to_string(), "false".to_string());
                        }
                        ArgTransform::Ref => {
                            let family = pointer_overloads
                                .family(arg.ty)
                                .ok_or_else(|| crate::Error::MissingTypeName(format!("pointer family for arg `{}` of method `{}`", arg.name, name)))?;
                            let ty_name = type_names
                                .name(family.by_ref)
                                .ok_or_else(|| crate::Error::MissingTypeName(format!("ref type for arg `{}` of method `{}`", arg.name, name)))?;
                            m.insert("ty".to_string(), ty_name.clone());
                            m.insert("is_ref".to_string(), "true".to_string());
                            m.insert("is_wrap".to_string(), "false".to_string());
                        }
                        ArgTransform::WrapDelegate => {
                            let family = delegate_overloads
                                .family(arg.ty)
                                .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate family for arg `{}` of method `{}`", arg.name, name)))?;
                            let sig_name = type_names
                                .name(family.signature)
                                .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate sig for arg `{}` of method `{}`", arg.name, name)))?;
                            let class_name = type_names
                                .name(family.class)
                                .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate class for arg `{}` of method `{}`", arg.name, name)))?;
                            m.insert("ty".to_string(), sig_name.clone());
                            m.insert("is_ref".to_string(), "false".to_string());
                            m.insert("is_wrap".to_string(), "true".to_string());
                            m.insert("wrapper_type".to_string(), class_name.clone());
                        }
                    }

                    args.push(m);
                }

                let Some(method_name) = method_names.get(method_fn_id) else { continue };

                let mut context = Context::new();
                context.insert("rval", rval);
                context.insert("method_name", &method_name);
                context.insert("interop_name", name);
                context.insert("is_void", &is_void);
                context.insert("args", &args);

                let rendered = templates.render("service/body_methods_overload.cs", &context)?;
                self.rendered.insert(method_fn_id, rendered);
            }
        }

        Ok(())
    }

    pub fn get(&self, fn_id: FunctionId) -> Option<&str> {
        self.rendered.get(&fn_id).map(|s| s.as_str())
    }
}
