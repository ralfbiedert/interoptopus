//! Renders body and async overload function declarations.
//!
//! Iterates `overload::all` for `Body` and `Async` overload kinds. For each,
//! resolves arg transforms (ref, delegate wrap) from type overload families and
//! renders via the shared `fns/overload/body.cs` template. Async overloads set
//! `is_async = true` which switches the template to trampoline + Task return.

use crate::lang::functions::overload::{ArgTransform, FnTransforms, OverloadKind, RvalTransform};
use crate::lang::functions::{Argument, Function};
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::lang::types::OverloadFamily;
use crate::output::{Output, OutputKind};
use crate::pass::{model, output, OutputResult, PassInfo};
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
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_imports: Default::default(), async_imports: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        overload_all: &model::fns::overload::all::Pass,
        originals: &model::fns::originals::Pass,
        types: &model::types::all::Pass,
        type_overloads: &model::types::overload::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut body = Vec::new();
            let mut asynk = Vec::new();

            for (&fn_id, original_fn) in originals.iter() {
                let Some(entries) = overload_all.overloads_for(fn_id) else { continue };

                for (_, kind) in entries {
                    match kind {
                        OverloadKind::Body(transforms) => {
                            body.push(render(original_fn, transforms, types, type_overloads, templates)?);
                        }
                        OverloadKind::Async(transforms) => {
                            asynk.push(render(original_fn, transforms, types, type_overloads, templates)?);
                        }
                        OverloadKind::Simple => {}
                    }
                }
            }

            body.sort();
            asynk.sort();
            self.body_imports.insert(file.clone(), body);
            self.async_imports.insert(file.clone(), asynk);
        }

        Ok(())
    }

    pub fn body_imports_for(&self, output: &Output) -> Option<&[String]> {
        self.body_imports.get(output).map(|s| s.as_slice())
    }

    pub fn async_imports_for(&self, output: &Output) -> Option<&[String]> {
        self.async_imports.get(output).map(|s| s.as_slice())
    }
}

fn render(
    original_fn: &Function,
    transforms: &FnTransforms,
    types: &model::types::all::Pass,
    type_overloads: &model::types::overload::all::Pass,
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

    // Return type
    let rval = if let RvalTransform::AsyncTask(result_ty_id) = transforms.rval {
        let result_ty = types.get(result_ty_id)
            .ok_or_else(|| crate::Error::MissingTypeName(format!("async result type for `{}`", name)))?;
        resolve_task_type(&result_ty.kind, types)
    } else {
        types.get(original_fn.signature.rval).map(|t| t.name.clone())
            .ok_or_else(|| crate::Error::MissingTypeName(format!("rval of overload `{}`", name)))?
    };

    let is_void = !is_async && matches!(
        types.get(original_fn.signature.rval).map(|t| &t.kind),
        Some(TypeKind::Primitive(Primitive::Void))
    );

    let native_rval_is_result = is_async && matches!(
        types.get(original_fn.signature.rval).map(|t| &t.kind),
        Some(TypeKind::TypePattern(TypePattern::Result(_, _, _)))
    );

    let mut context = Context::new();
    context.insert("name", name);
    context.insert("rval", &rval);
    context.insert("is_void", &is_void);
    context.insert("is_async", &is_async);
    context.insert("has_wraps", &has_wraps);
    context.insert("args", &args);
    context.insert("native_args", &native_args);
    context.insert("native_rval_is_result", &native_rval_is_result);

    if let RvalTransform::AsyncTask(result_ty_id) = transforms.rval {
        if let Some(result_ty) = types.get(result_ty_id) {
            let trampoline_field = format!("_trampoline{}", result_ty.name);
            context.insert("trampoline_field", &trampoline_field);
        }
    }

    templates.render("fns/overload/body.cs", &context).map_err(Into::into)
}

fn resolve_args(
    args: &[Argument],
    transforms: &[ArgTransform],
    types: &model::types::all::Pass,
    type_overloads: &model::types::overload::all::Pass,
    fn_name: &str,
) -> Result<(Vec<HashMap<&'static str, Value>>, bool), crate::Error> {
    let mut out = Vec::new();
    let mut has_wraps = false;

    for (arg, transform) in args.iter().zip(transforms) {
        let mut m = HashMap::new();
        m.insert("name", Value::String(arg.name.clone()));

        match transform {
            ArgTransform::PassThrough => {
                let ty_name = types.get(arg.ty).map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("arg `{}` of overload `{}`", arg.name, fn_name)))?;
                m.insert("ty", Value::String(ty_name.clone()));
                m.insert("is_ref", Value::String("false".into()));
                m.insert("is_wrap", Value::String("false".into()));
            }
            ArgTransform::Ref => {
                let family = match type_overloads.get(arg.ty) {
                    Some(OverloadFamily::Pointer(f)) => f,
                    _ => return Err(crate::Error::MissingTypeName(format!("pointer family for arg `{}` of overload `{}`", arg.name, fn_name))),
                };
                let ty_name = types.get(family.by_ref).map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("ref type for arg `{}` of overload `{}`", arg.name, fn_name)))?;
                m.insert("ty", Value::String(ty_name.clone()));
                m.insert("is_ref", Value::String("true".into()));
                m.insert("is_wrap", Value::String("false".into()));
            }
            ArgTransform::WrapDelegate => {
                let family = match type_overloads.get(arg.ty) {
                    Some(OverloadFamily::Delegate(f)) => f,
                    _ => return Err(crate::Error::MissingTypeName(format!("delegate family for arg `{}` of overload `{}`", arg.name, fn_name))),
                };
                let sig_name = types.get(family.signature).map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate sig for arg `{}` of overload `{}`", arg.name, fn_name)))?;
                let class_name = types.get(family.class).map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("delegate class for arg `{}` of overload `{}`", arg.name, fn_name)))?;
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

fn resolve_task_type(result_kind: &TypeKind, types: &model::types::all::Pass) -> String {
    match result_kind {
        TypeKind::TypePattern(TypePattern::Result(ok_ty, _, _)) => {
            let ok_kind = types.get(*ok_ty).map(|t| &t.kind);
            if matches!(ok_kind, Some(TypeKind::Primitive(Primitive::Void))) {
                "Task".to_string()
            } else {
                let ok_name = types.get(*ok_ty).map(|t| t.name.clone()).unwrap_or_else(|| "void".to_string());
                format!("Task<{}>", ok_name)
            }
        }
        _ => "Task".to_string(),
    }
}
