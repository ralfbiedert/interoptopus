//! Renders body and async overload function declarations.
//!
//! Iterates `overload::all` for `Body` and `Async` overload kinds. For each,
//! resolves arg transforms (ref, delegate wrap) from type overload families and
//! renders the appropriate template. Body overloads use `fns/overload/body.cs`,
//! async overloads use `fns/overload/asynk.cs`.
//!
//! Both kinds share the same arg-transform resolution logic.

use crate::lang::functions::overload::{ArgTransform, FnTransforms, OverloadKind, RvalTransform};
use crate::lang::functions::{Argument, Function};
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::lang::types::OverloadFamily;
use crate::output::{Output, OutputKind};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::{Context, TemplateEngine};
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
                            body.push(render_body(original_fn, transforms, types, type_overloads, templates)?);
                        }
                        OverloadKind::Async(transforms) => {
                            asynk.push(render_async(original_fn, transforms, types, type_overloads, templates)?);
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

// ── Shared arg resolution ────────────────────────────────────────────────

fn resolve_args(
    args: &[Argument],
    transforms: &[ArgTransform],
    types: &model::types::all::Pass,
    type_overloads: &model::types::overload::all::Pass,
    fn_name: &str,
) -> Result<(Vec<HashMap<String, String>>, bool), crate::Error> {
    let mut out = Vec::new();
    let mut has_wraps = false;

    for (arg, transform) in args.iter().zip(transforms) {
        let mut m = HashMap::new();
        m.insert("name".to_string(), arg.name.clone());

        match transform {
            ArgTransform::PassThrough => {
                let ty_name = types.get(arg.ty).map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("arg `{}` of overload `{}`", arg.name, fn_name)))?;
                m.insert("ty".to_string(), ty_name.clone());
                m.insert("is_ref".to_string(), "false".to_string());
                m.insert("is_wrap".to_string(), "false".to_string());
            }
            ArgTransform::Ref => {
                let family = match type_overloads.get(arg.ty) {
                    Some(OverloadFamily::Pointer(f)) => f,
                    _ => return Err(crate::Error::MissingTypeName(format!("pointer family for arg `{}` of overload `{}`", arg.name, fn_name))),
                };
                let ty_name = types.get(family.by_ref).map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("ref type for arg `{}` of overload `{}`", arg.name, fn_name)))?;
                m.insert("ty".to_string(), ty_name.clone());
                m.insert("is_ref".to_string(), "true".to_string());
                m.insert("is_wrap".to_string(), "false".to_string());
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
                m.insert("ty".to_string(), sig_name.clone());
                m.insert("is_ref".to_string(), "false".to_string());
                m.insert("is_wrap".to_string(), "true".to_string());
                m.insert("wrapper_type".to_string(), class_name.clone());
                has_wraps = true;
            }
        }
        out.push(m);
    }

    Ok((out, has_wraps))
}

// ── Body rendering ───────────────────────────────────────────────────────

fn render_body(
    original_fn: &Function,
    transforms: &FnTransforms,
    types: &model::types::all::Pass,
    type_overloads: &model::types::overload::all::Pass,
    templates: &TemplateEngine,
) -> Result<String, crate::Error> {
    let name = &original_fn.name;
    let rval = types.get(original_fn.signature.rval).map(|t| &t.name)
        .ok_or_else(|| crate::Error::MissingTypeName(format!("rval of body overload `{}`", name)))?;
    let is_void = matches!(types.get(original_fn.signature.rval).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));

    let (args, _) = resolve_args(&original_fn.signature.arguments, &transforms.args, types, type_overloads, name)?;

    let mut context = Context::new();
    context.insert("name", name);
    context.insert("rval", rval);
    context.insert("is_void", &is_void);
    context.insert("args", &args);
    templates.render("fns/overload/body.cs", &context).map_err(Into::into)
}

// ── Async rendering ──────────────────────────────────────────────────────

fn render_async(
    original_fn: &Function,
    transforms: &FnTransforms,
    types: &model::types::all::Pass,
    type_overloads: &model::types::overload::all::Pass,
    templates: &TemplateEngine,
) -> Result<String, crate::Error> {
    let RvalTransform::AsyncTask(async_result_ty) = transforms.rval else {
        return Err(crate::Error::MissingTypeName("async overload without AsyncTask rval".to_string()));
    };

    let name = &original_fn.name;

    let result_ty = types.get(async_result_ty)
        .ok_or_else(|| crate::Error::MissingTypeName(format!("async result type for `{}`", name)))?;
    let result_ty_name = &result_ty.name;
    let task_rval = resolve_task_type(&result_ty.kind, types);
    let trampoline_field = format!("_trampoline{}", result_ty_name);

    // Args = all original args except the last (callback)
    let non_callback_args = &original_fn.signature.arguments[..original_fn.signature.arguments.len() - 1];
    let (args, has_wraps) = resolve_args(non_callback_args, &transforms.args, types, type_overloads, name)?;

    // Build native call arg names with transforms applied
    let mut native_args: Vec<HashMap<String, String>> = Vec::new();
    for (arg, transform) in non_callback_args.iter().zip(&transforms.args) {
        let mut m = HashMap::new();
        let forwarded = match transform {
            ArgTransform::WrapDelegate => format!("{}_wrapped", arg.name),
            ArgTransform::Ref => format!("ref {}", arg.name),
            ArgTransform::PassThrough => arg.name.clone(),
        };
        m.insert("name".to_string(), forwarded);
        native_args.push(m);
    }

    let native_rval_is_result = matches!(
        types.get(original_fn.signature.rval).map(|t| &t.kind),
        Some(TypeKind::TypePattern(TypePattern::Result(_, _, _)))
    );

    let mut context = Context::new();
    context.insert("name", name);
    context.insert("task_rval", &task_rval);
    context.insert("trampoline_field", &trampoline_field);
    context.insert("args", &args);
    context.insert("native_args", &native_args);
    context.insert("native_rval_is_result", &native_rval_is_result);
    context.insert("has_wraps", &has_wraps);
    templates.render("fns/overload/asynk.cs", &context).map_err(Into::into)
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
