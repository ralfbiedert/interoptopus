//! Renders async function overloads that return `Task<T>`.
//!
//! For each function with an `Async` overload in `overload::all`, this renders
//! an overload that creates a trampoline call, applies body-style arg transforms
//! (ref, delegate wrap) to remaining args, invokes the native function, and
//! returns a `Task<T>`.

use crate::lang::functions::overload::{ArgTransform, OverloadKind, RvalTransform};
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::lang::types::OverloadFamily;
use crate::output::{Output, OutputKind};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
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
        overload_all: &model::fns::overload::all::Pass,
        originals: &model::fns::originals::Pass,
        types: &model::types::all::Pass,
        type_overloads: &model::types::overload::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut imports = Vec::new();

            for (&fn_id, original_fn) in originals.iter() {
                let Some(overload_entries) = overload_all.overloads_for(fn_id) else { continue };

                for (_, kind) in overload_entries {
                    let OverloadKind::Async(transforms) = kind else { continue };
                    let RvalTransform::AsyncTask(async_result_ty) = transforms.rval else { continue };

                    let name = &original_fn.name;

                    // Get the async Result type name (for trampoline field naming)
                    let result_ty = types.get(async_result_ty).ok_or_else(|| {
                        crate::Error::MissingTypeName(format!("async result type for `{}`", name))
                    })?;
                    let result_ty_name = &result_ty.name;

                    // Extract Ok type from Result<Ok, Err> to determine Task<T>
                    let task_rval = resolve_task_type(&result_ty.kind, types);

                    // Trampoline field name
                    let trampoline_field = format!("_trampoline{}", result_ty_name);

                    // Build arg list (all args except the callback) with transforms
                    let non_callback_args = &original_fn.signature.arguments[..original_fn.signature.arguments.len() - 1];
                    let mut args: Vec<HashMap<String, String>> = Vec::new();
                    let mut has_wraps = false;

                    for (arg, transform) in non_callback_args.iter().zip(&transforms.args) {
                        let mut m = HashMap::new();
                        m.insert("name".to_string(), arg.name.clone());

                        match transform {
                            ArgTransform::PassThrough => {
                                let ty_name = types.get(arg.ty).map(|t| &t.name).ok_or_else(|| {
                                    crate::Error::MissingTypeName(format!("arg `{}` of async overload `{}`", arg.name, name))
                                })?;
                                m.insert("ty".to_string(), ty_name.clone());
                                m.insert("is_ref".to_string(), "false".to_string());
                                m.insert("is_wrap".to_string(), "false".to_string());
                            }
                            ArgTransform::Ref => {
                                let family = match type_overloads.get(arg.ty) {
                                    Some(OverloadFamily::Pointer(f)) => f,
                                    _ => return Err(crate::Error::MissingTypeName(format!("pointer family for arg `{}` of async overload `{}`", arg.name, name))),
                                };
                                let ty_name = types.get(family.by_ref).map(|t| &t.name).ok_or_else(|| {
                                    crate::Error::MissingTypeName(format!("ref type for arg `{}` of async overload `{}`", arg.name, name))
                                })?;
                                m.insert("ty".to_string(), ty_name.clone());
                                m.insert("is_ref".to_string(), "true".to_string());
                                m.insert("is_wrap".to_string(), "false".to_string());
                            }
                            ArgTransform::WrapDelegate => {
                                let family = match type_overloads.get(arg.ty) {
                                    Some(OverloadFamily::Delegate(f)) => f,
                                    _ => return Err(crate::Error::MissingTypeName(format!("delegate family for arg `{}` of async overload `{}`", arg.name, name))),
                                };
                                let sig_name = types.get(family.signature).map(|t| &t.name).ok_or_else(|| {
                                    crate::Error::MissingTypeName(format!("delegate sig for arg `{}` of async overload `{}`", arg.name, name))
                                })?;
                                let class_name = types.get(family.class).map(|t| &t.name).ok_or_else(|| {
                                    crate::Error::MissingTypeName(format!("delegate class for arg `{}` of async overload `{}`", arg.name, name))
                                })?;
                                m.insert("ty".to_string(), sig_name.clone());
                                m.insert("is_ref".to_string(), "false".to_string());
                                m.insert("is_wrap".to_string(), "true".to_string());
                                m.insert("wrapper_type".to_string(), class_name.clone());
                                has_wraps = true;
                            }
                        }
                        args.push(m);
                    }

                    // Native call arg names (original arg names, applying transforms)
                    let mut native_args: Vec<HashMap<String, String>> = Vec::new();
                    for (arg, transform) in non_callback_args.iter().zip(&transforms.args) {
                        let mut m = HashMap::new();
                        match transform {
                            ArgTransform::WrapDelegate => {
                                m.insert("name".to_string(), format!("{}_wrapped", arg.name));
                            }
                            ArgTransform::Ref => {
                                m.insert("name".to_string(), format!("ref {}", arg.name));
                            }
                            _ => {
                                m.insert("name".to_string(), arg.name.clone());
                            }
                        }
                        native_args.push(m);
                    }

                    // Determine if the native rval is a Result type (needs .AsOk())
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

                    let rendered = templates.render("fns/overload/asynk.cs", &context)?;
                    imports.push(rendered);
                }
            }

            imports.sort();
            self.fn_imports.insert(file.clone(), imports);
        }

        Ok(())
    }

    pub fn imports_for(&self, output: &Output) -> Option<&[String]> {
        self.fn_imports.get(output).map(|s| s.as_slice())
    }
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
