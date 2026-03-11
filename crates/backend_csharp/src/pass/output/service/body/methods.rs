//! Renders all service method variants per service.
//!
//! For each service method, renders a base forwarding method (using ref overload
//! types if available, otherwise original types) and optionally an additional
//! delegate overload. All methods simply forward to the corresponding
//! `Interop.function_name` — overload resolution in C# picks the right variant.

use crate::lang::functions::overload::ArgTransform;
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
        fn_map: &model::fns::all::Pass,
        type_names: &model::types::names::Pass,
        type_kinds: &model::types::kind::Pass,
        method_names: &model::service::method::names::Pass,
        overload_simple: &model::fns::overload::simple::Pass,
        overload_body: &model::fns::overload::body::Pass,
        pointer_overloads: &model::types::overload::pointer::Pass,
        delegate_overloads: &model::types::overload::delegate::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (service_id, service) in service_map.iter() {
            let mut rendered_methods = Vec::new();

            for &method_fn_id in &service.methods {
                let Some(method_fn) = fn_map.get(method_fn_id) else { continue };
                let Some(method_name) = method_names.get(method_fn_id) else { continue };

                // Base method: use simple (ref) overload if available, otherwise original types.
                if let Some(rendered) = self.render_ref(templates, method_fn_id, method_name, fn_map, type_names, type_kinds, overload_simple)? {
                    rendered_methods.push(rendered);
                } else if let Some(rendered) = self.render_plain(templates, method_fn_id, method_name, method_fn, type_names, type_kinds)? {
                    rendered_methods.push(rendered);
                }

                // Additional delegate overload if body transforms exist.
                if let Some(rendered) =
                    self.render_delegate(templates, method_fn_id, method_name, method_fn, type_names, type_kinds, overload_body, pointer_overloads, delegate_overloads)?
                {
                    rendered_methods.push(rendered);
                }
            }

            self.body_methods.insert(*service_id, rendered_methods);
        }

        Ok(())
    }

    fn render_plain(
        &self,
        templates: &interoptopus_backends::template::TemplateEngine,
        method_fn_id: crate::lang::FunctionId,
        method_name: &str,
        method_fn: &crate::lang::functions::Function,
        type_names: &model::types::names::Pass,
        type_kinds: &model::types::kind::Pass,
    ) -> Result<Option<String>, crate::Error> {
        let Some(rval) = type_names.name(method_fn.signature.rval) else { return Ok(None) };
        let is_void = matches!(type_kinds.get(method_fn.signature.rval), Some(TypeKind::Primitive(Primitive::Void)));

        let mut args: Vec<HashMap<&str, Value>> = Vec::new();
        for arg in method_fn.signature.arguments.iter().skip(1) {
            let Some(arg_ty) = type_names.name(arg.ty) else { return Ok(None) };
            let mut m = HashMap::new();
            m.insert("name", Value::String(arg.name.clone()));
            m.insert("ty", Value::String(arg_ty.clone()));
            m.insert("is_ref", Value::Bool(false));
            args.push(m);
        }

        let mut context = Context::new();
        context.insert("rval", rval);
        context.insert("is_void", &is_void);
        context.insert("method_name", &method_name);
        context.insert("interop_name", &method_fn.name);
        context.insert("args", &args);

        Ok(Some(templates.render("service/body_methods.cs", &context)?))
    }

    fn render_ref(
        &self,
        templates: &interoptopus_backends::template::TemplateEngine,
        method_fn_id: crate::lang::FunctionId,
        method_name: &str,
        fn_map: &model::fns::all::Pass,
        type_names: &model::types::names::Pass,
        type_kinds: &model::types::kind::Pass,
        overload_simple: &model::fns::overload::simple::Pass,
    ) -> Result<Option<String>, crate::Error> {
        let Some(overload_ids) = overload_simple.overloads_for(method_fn_id) else {
            return Ok(None);
        };
        let Some(&simple_id) = overload_ids.first() else { return Ok(None) };
        let Some(simple_fn) = fn_map.get(simple_id) else { return Ok(None) };
        let Some(rval) = type_names.name(simple_fn.signature.rval) else { return Ok(None) };
        let is_void = matches!(type_kinds.get(simple_fn.signature.rval), Some(TypeKind::Primitive(Primitive::Void)));

        let mut args: Vec<HashMap<&str, Value>> = Vec::new();
        for arg in simple_fn.signature.arguments.iter().skip(1) {
            let Some(arg_ty) = type_names.name(arg.ty) else { return Ok(None) };
            let is_ref = matches!(type_kinds.get(arg.ty), Some(TypeKind::Pointer(p)) if p.kind == PointerKind::ByRef);
            let mut m = HashMap::new();
            m.insert("name", Value::String(arg.name.clone()));
            m.insert("ty", Value::String(arg_ty.clone()));
            m.insert("is_ref", Value::Bool(is_ref));
            args.push(m);
        }

        let mut context = Context::new();
        context.insert("rval", rval);
        context.insert("is_void", &is_void);
        context.insert("method_name", &method_name);
        context.insert("interop_name", &simple_fn.name);
        context.insert("args", &args);

        Ok(Some(templates.render("service/body_methods.cs", &context)?))
    }

    fn render_delegate(
        &self,
        templates: &interoptopus_backends::template::TemplateEngine,
        method_fn_id: crate::lang::FunctionId,
        method_name: &str,
        method_fn: &crate::lang::functions::Function,
        type_names: &model::types::names::Pass,
        type_kinds: &model::types::kind::Pass,
        overload_body: &model::fns::overload::body::Pass,
        pointer_overloads: &model::types::overload::pointer::Pass,
        delegate_overloads: &model::types::overload::delegate::Pass,
    ) -> Result<Option<String>, crate::Error> {
        let Some(transforms) = overload_body.transforms(method_fn_id) else {
            return Ok(None);
        };
        let name = &method_fn.name;

        let Some(rval) = type_names.name(method_fn.signature.rval) else { return Ok(None) };
        let is_void = matches!(type_kinds.get(method_fn.signature.rval), Some(TypeKind::Primitive(Primitive::Void)));

        let mut args: Vec<HashMap<&str, Value>> = Vec::new();
        for (arg, transform) in method_fn.signature.arguments.iter().zip(&transforms.args).skip(1) {
            let mut m = HashMap::new();
            m.insert("name", Value::String(arg.name.clone()));

            match transform {
                ArgTransform::PassThrough => {
                    let ty_name = type_names
                        .name(arg.ty)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("arg `{}` of method `{}`", arg.name, name)))?;
                    m.insert("ty", Value::String(ty_name.clone()));
                    m.insert("is_ref", Value::Bool(false));
                }
                ArgTransform::Ref => {
                    let family = pointer_overloads
                        .family(arg.ty)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("pointer family for arg `{}` of method `{}`", arg.name, name)))?;
                    let ty_name = type_names
                        .name(family.by_ref)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("ref type for arg `{}` of method `{}`", arg.name, name)))?;
                    m.insert("ty", Value::String(ty_name.clone()));
                    m.insert("is_ref", Value::Bool(matches!(type_kinds.get(family.by_ref), Some(TypeKind::Pointer(p)) if p.kind == PointerKind::ByRef)));
                }
                ArgTransform::WrapDelegate => {
                    let family = delegate_overloads
                        .family(arg.ty)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate family for arg `{}` of method `{}`", arg.name, name)))?;
                    let sig_name = type_names
                        .name(family.signature)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate sig for arg `{}` of method `{}`", arg.name, name)))?;
                    m.insert("ty", Value::String(sig_name.clone()));
                    m.insert("is_ref", Value::Bool(false));
                }
            }

            args.push(m);
        }

        let mut context = Context::new();
        context.insert("rval", rval);
        context.insert("is_void", &is_void);
        context.insert("method_name", &method_name);
        context.insert("interop_name", name);
        context.insert("args", &args);

        Ok(Some(templates.render("service/body_methods.cs", &context)?))
    }

    pub fn get(&self, service_id: ServiceId) -> Option<&[String]> {
        self.body_methods.get(&service_id).map(|v| v.as_slice())
    }
}
