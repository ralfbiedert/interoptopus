//! Renders body overload function declarations.
//!
//! Body overloads have a method body that wraps delegate arguments into their
//! class, calls the target function, and disposes the wrappers. This pass
//! resolves all names and types from the originals and type overload passes,
//! guided by the per-argument transforms from the body overload model pass.

use crate::lang::functions::overload::ArgTransform;
use crate::lang::types::{Primitive, TypeKind};
use crate::output::{Output, OutputKind};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::{Context, TemplateEngine};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    fn_imports: HashMap<Output, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, fn_imports: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        overload_body: &model::fns::overload::body::Pass,
        originals: &model::fns::originals::Pass,
        types: &model::types::all::Pass,
        pointer_overloads: &model::types::overload::pointer::Pass,
        delegate_overloads: &model::types::overload::delegate::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut imports = Vec::new();

            for (fn_id, transforms) in overload_body.iter() {
                let Some(original_fn) = originals.get(fn_id) else { continue };
                let rendered = render_body_overload(original_fn, transforms, types, pointer_overloads, delegate_overloads, templates)?;
                imports.push(rendered);
            }

            imports.sort();

            self.fn_imports.insert(output.clone(), imports);
        }

        Ok(())
    }

    pub fn imports_for(&self, output: &Output) -> Option<&[String]> {
        self.fn_imports.get(output).map(|s| s.as_slice())
    }
}

fn render_body_overload(
    original_fn: &crate::lang::functions::Function,
    transforms: &crate::lang::functions::overload::FnTransforms,
    types: &model::types::all::Pass,
    pointer_overloads: &model::types::overload::pointer::Pass,
    delegate_overloads: &model::types::overload::delegate::Pass,
    templates: &TemplateEngine,
) -> Result<String, crate::Error> {
    let name = &original_fn.name;

    let rval = types.name(original_fn.signature.rval)
        .ok_or_else(|| crate::Error::MissingTypeName(format!("rval of body overload `{}`", name)))?;

    let is_void = matches!(types.kind(original_fn.signature.rval), Some(TypeKind::Primitive(Primitive::Void)));

    let mut args: Vec<HashMap<String, String>> = Vec::new();
    for (arg, transform) in original_fn.signature.arguments.iter().zip(&transforms.args) {
        let mut m = HashMap::new();
        m.insert("name".to_string(), arg.name.clone());

        match transform {
            ArgTransform::PassThrough => {
                let ty_name = types.name(arg.ty)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("arg `{}` of body overload `{}`", arg.name, name)))?;
                m.insert("ty".to_string(), ty_name.clone());
                m.insert("is_ref".to_string(), "false".to_string());
                m.insert("is_wrap".to_string(), "false".to_string());
            }
            ArgTransform::Ref => {
                let family = pointer_overloads
                    .get(arg.ty)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("pointer family for arg `{}` of body overload `{}`", arg.name, name)))?;
                let ty_name = types.name(family.by_ref)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("ref type for arg `{}` of body overload `{}`", arg.name, name)))?;
                m.insert("ty".to_string(), ty_name.clone());
                m.insert("is_ref".to_string(), "true".to_string());
                m.insert("is_wrap".to_string(), "false".to_string());
            }
            ArgTransform::WrapDelegate => {
                let family = delegate_overloads
                    .get(arg.ty)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate family for arg `{}` of body overload `{}`", arg.name, name)))?;
                let sig_name = types.name(family.signature)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate sig for arg `{}` of body overload `{}`", arg.name, name)))?;
                let class_name = types.name(family.class)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate class for arg `{}` of body overload `{}`", arg.name, name)))?;
                m.insert("ty".to_string(), sig_name.clone());
                m.insert("is_ref".to_string(), "false".to_string());
                m.insert("is_wrap".to_string(), "true".to_string());
                m.insert("wrapper_type".to_string(), class_name.clone());
            }
        }

        args.push(m);
    }

    let mut context = Context::new();
    context.insert("name", name);
    context.insert("rval", rval);
    context.insert("is_void", &is_void);
    context.insert("args", &args);

    templates.render("fns/overload/body.cs", &context).map_err(Into::into)
}
