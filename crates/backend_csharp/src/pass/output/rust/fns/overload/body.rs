//! Renders body and async overload function declarations.
//!
//! Iterates `fns::all` for overloads with `Body` and `Async` kinds. For each,
//! resolves arg transforms from the `Overload` data stored in the function's
//! `FunctionKind`, and renders via the shared `fns/overload/body.cs` template.
//! Async overloads set `is_async = true` which switches the template to
//! trampoline + Task return.

use crate::lang::functions::FunctionKind;
use crate::lang::functions::overload::{ArgTransform, FnTransforms, OverloadKind, RvalTransform};
use crate::lang::functions::{Argument, Function};
use crate::lang::types::OverloadFamily;
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, format_docs, model, output};
use interoptopus_backends::template::{Context, TemplateEngine, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_imports: HashMap<Output, Vec<String>>,
    async_imports: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_imports: HashMap::default(), async_imports: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        type_overloads: &model::rust::types::overload::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut body = Vec::new();
            let mut asynk = Vec::new();

            for (&overload_id, function) in fns_all.overloads() {
                let FunctionKind::Overload(ref overload) = function.kind else { continue };

                if !output_master.fn_belongs_to(overload.base, file) {
                    continue;
                }

                // Look up the original function for context (native args, rval)
                let Some(original_fn) = fns_all.get(overload.base) else { continue };

                match &overload.kind {
                    OverloadKind::Body(transforms) => {
                        body.push(render(original_fn, function, transforms, types, type_overloads, templates)?);
                    }
                    OverloadKind::Async(transforms) => {
                        asynk.push(render(original_fn, function, transforms, types, type_overloads, templates)?);
                    }
                    OverloadKind::Simple => {}
                }
            }

            body.sort();
            asynk.sort();
            self.body_imports.insert(file.clone(), body);
            self.async_imports.insert(file.clone(), asynk);
        }

        Ok(())
    }

    #[must_use]
    pub fn body_imports_for(&self, output: &Output) -> Option<&[String]> {
        self.body_imports.get(output).map(std::vec::Vec::as_slice)
    }

    #[must_use]
    pub fn async_imports_for(&self, output: &Output) -> Option<&[String]> {
        self.async_imports.get(output).map(std::vec::Vec::as_slice)
    }
}

fn render(
    original_fn: &Function,
    overload_fn: &Function,
    transforms: &FnTransforms,
    types: &model::common::types::all::Pass,
    type_overloads: &model::rust::types::overload::all::Pass,
    templates: &TemplateEngine,
) -> Result<String, crate::Error> {
    let name = &original_fn.name;
    let is_async = matches!(transforms.rval, RvalTransform::AsyncTask(_));

    // For async: args exclude the callback (last); for body: all args
    let original_args = if is_async {
        &original_fn.signature.arguments[..original_fn.signature.arguments.len() - 1]
    } else {
        &original_fn.signature.arguments[..]
    };

    // Resolve overloaded arg types + detect wraps
    let (args, has_wraps) = resolve_args(original_args, &transforms.args, types, type_overloads, name)?;

    // Build native call forwarding names (applying ref/wrap transforms)
    let native_args = build_native_args(original_args, &transforms.args);

    // Return type: use the overload function's rval directly (Task type for async, original for body)
    let rval = types
        .get(overload_fn.signature.rval)
        .map(|t| t.name.clone())
        .ok_or_else(|| crate::Error::from(format!("rval of overload `{name}`")))?;

    let is_void = !is_async && matches!(types.get(original_fn.signature.rval).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));

    let native_rval_is_result = is_async && matches!(types.get(original_fn.signature.rval).map(|t| &t.kind), Some(TypeKind::TypePattern(TypePattern::Result(_, _, _))));

    let docs = format_docs(&overload_fn.docs);
    let mut context = Context::new();
    context.insert("name", name);
    context.insert("rval", &rval);
    context.insert("is_void", &is_void);
    context.insert("is_async", &is_async);
    context.insert("has_wraps", &has_wraps);
    context.insert("args", &args);
    context.insert("native_args", &native_args);
    context.insert("native_rval_is_result", &native_rval_is_result);
    context.insert("docs", &docs);

    if let RvalTransform::AsyncTask(result_ty_id) = transforms.rval
        && let Some(result_ty) = types.get(result_ty_id)
    {
        let trampoline_field = format!("_trampoline{}", result_ty.name);
        context.insert("trampoline_field", &trampoline_field);
    }

    templates.render("rust/fns/overload/body.cs", &context).map_err(Into::into)
}

fn resolve_args(
    args: &[Argument],
    transforms: &[ArgTransform],
    types: &model::common::types::all::Pass,
    type_overloads: &model::rust::types::overload::all::Pass,
    fn_name: &str,
) -> Result<(Vec<HashMap<&'static str, Value>>, bool), crate::Error> {
    let mut out = Vec::new();
    let mut has_wraps = false;

    for (arg, transform) in args.iter().zip(transforms) {
        let mut m = HashMap::new();
        m.insert("name", Value::String(arg.name.clone()));

        match transform {
            ArgTransform::PassThrough => {
                let arg_type = types
                    .get(arg.ty)
                    .ok_or_else(|| crate::Error::from(format!("arg `{}` of overload `{}`", arg.name, fn_name)))?;
                let decorated = match &arg_type.decorators.param {
                    Some(d) => format!("{d} {}", arg_type.name),
                    None => arg_type.name.clone(),
                };
                m.insert("ty", Value::String(decorated));
                m.insert("is_ref", Value::String("false".into()));
                m.insert("is_wrap", Value::String("false".into()));
            }
            ArgTransform::Ref => {
                let Some(OverloadFamily::Pointer(family)) = type_overloads.get(arg.ty) else {
                    return Err(crate::Error::from(format!("pointer family for arg `{}` of overload `{}`", arg.name, fn_name)));
                };
                let arg_type = types
                    .get(family.by_ref)
                    .ok_or_else(|| crate::Error::from(format!("ref type for arg `{}` of overload `{}`", arg.name, fn_name)))?;
                let decorated = match &arg_type.decorators.param {
                    Some(d) => format!("{d} {}", arg_type.name),
                    None => arg_type.name.clone(),
                };
                m.insert("ty", Value::String(decorated));
                m.insert("is_ref", Value::String("true".into()));
                m.insert("is_wrap", Value::String("false".into()));
            }
            ArgTransform::WrapDelegate => {
                let Some(OverloadFamily::Delegate(family)) = type_overloads.get(arg.ty) else {
                    return Err(crate::Error::from(format!("delegate family for arg `{}` of overload `{}`", arg.name, fn_name)));
                };
                let sig_name = types
                    .get(family.signature)
                    .map(|t| &t.name)
                    .ok_or_else(|| crate::Error::from(format!("delegate sig for arg `{}` of overload `{}`", arg.name, fn_name)))?;
                let class_name = types
                    .get(family.class)
                    .map(|t| &t.name)
                    .ok_or_else(|| crate::Error::from(format!("delegate class for arg `{}` of overload `{}`", arg.name, fn_name)))?;
                m.insert("ty", Value::String(sig_name.clone()));
                m.insert("is_ref", Value::String("false".into()));
                m.insert("is_wrap", Value::String("true".into()));
                m.insert("wrapper_type", Value::String(class_name.clone()));
                has_wraps = true;
            }
        }
        out.push(m);
    }

    Ok((out, has_wraps))
}

fn build_native_args(args: &[Argument], transforms: &[ArgTransform]) -> Vec<HashMap<&'static str, Value>> {
    args.iter()
        .zip(transforms)
        .map(|(arg, transform)| {
            let forwarded = match transform {
                ArgTransform::WrapDelegate => format!("{}_wrapped", arg.name),
                ArgTransform::Ref => format!("ref {}", arg.name),
                ArgTransform::PassThrough => arg.name.clone(),
            };
            let mut m = HashMap::new();
            m.insert("name", Value::String(forwarded));
            m
        })
        .collect()
}
