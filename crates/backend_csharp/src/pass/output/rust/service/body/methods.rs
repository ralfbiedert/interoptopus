//! Renders all service method variants per service.
//!
//! For each function ID in `service.methods`, renders the appropriate method variant.
//! Original functions produce a base forwarding method. Overloads produce either a
//! regular overloaded method or an async method, depending on their `OverloadKind`.
//!
//! All decisions about which overloads to include have already been made by the
//! `service::method::overload` model pass — this output pass simply renders what
//! the model provides.

use crate::lang::ServiceId;
use crate::lang::functions::FunctionKind;
use crate::lang::functions::overload::{OverloadKind, RvalTransform};
use crate::lang::types::kind::{PointerKind, Primitive, TypeKind, TypePattern};
use crate::pass::{OutputResult, PassInfo, format_docs, model, output};
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
        output_master: &output::common::master::Pass,
        services: &model::common::service::all::Pass,
        fns: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        method_names: &model::rust::service::method::names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (service_id, service) in services.iter() {
            let mut rendered_methods = Vec::new();

            for &method_fn_id in &service.methods {
                let Some(method_fn) = fns.get(method_fn_id) else { continue };
                let Some(method_name) = method_names.get(method_fn_id) else { continue };

                match &method_fn.kind {
                    // Skip originals that have overloads — the overload will be rendered instead.
                    // Only render originals that have no overloads (no body/async transform needed).
                    FunctionKind::Original => {
                        let has_overload = fns.overloads_for(method_fn_id).next().is_some();
                        if has_overload {
                            continue;
                        }
                        let rval_kind = types.get(method_fn.signature.rval).map(|t| &t.kind);
                        let result_info = resolve_result_rval(rval_kind, types);

                        let rval = result_info
                            .rval_name
                            .as_deref()
                            .or_else(|| types.get(method_fn.signature.rval).map(|t| t.name.as_str()));
                        let Some(rval) = rval else { continue };
                        let is_void = result_info.is_void || matches!(rval_kind, Some(TypeKind::Primitive(Primitive::Void)));

                        let docs = format_docs(&method_fn.docs);
                        let args = build_args(&method_fn.signature.arguments[1..], types);
                        rendered_methods.push(render(templates, rval, is_void, result_info.as_ok, method_name, &method_fn.name, &args, &docs, "public", "_context")?);
                    }
                    FunctionKind::Overload(overload) => {
                        let Some(original_fn) = fns.get(overload.base) else { continue };
                        let Some(base_method_name) = method_names.get(overload.base) else { continue };

                        let docs = format_docs(&method_fn.docs);
                        let self_arg = if has_service_self_arg(method_fn, types) { "this" } else { "_context" };

                        if let OverloadKind::Async(transforms) = &overload.kind {
                            let task_rval = types.get(method_fn.signature.rval).map_or_else(|| "Task".to_string(), |t| t.name.clone());
                            let async_args = build_args(&method_fn.signature.arguments[1..], types);

                            let (trampoline_field, is_task_void) = if let RvalTransform::AsyncTask(result_ty_id) = transforms.rval {
                                let result_ty = types.get(result_ty_id);
                                let field = result_ty.map_or_else(|| "_trampoline".to_string(), |t| format!("_trampoline{}", t.name));
                                let is_void = task_rval == "Task";
                                (field, is_void)
                            } else {
                                ("_trampoline".to_string(), false)
                            };

                            rendered_methods.push(render_async(templates, &task_rval, base_method_name, &original_fn.name, &async_args, &docs, "public", self_arg, &trampoline_field, is_task_void)?);
                        } else {
                            let rval_kind = types.get(original_fn.signature.rval).map(|t| &t.kind);
                            let result_info = resolve_result_rval(rval_kind, types);

                            let rval = result_info
                                .rval_name
                                .as_deref()
                                .or_else(|| types.get(original_fn.signature.rval).map(|t| t.name.as_str()));
                            let Some(rval) = rval else { continue };
                            let is_void = result_info.is_void || matches!(rval_kind, Some(TypeKind::Primitive(Primitive::Void)));

                            let overload_args = build_args(&method_fn.signature.arguments[1..], types);
                            rendered_methods.push(render(
                                templates,
                                rval,
                                is_void,
                                result_info.as_ok,
                                base_method_name,
                                &method_fn.name,
                                &overload_args,
                                &docs,
                                "public",
                                self_arg,
                            )?);
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

fn build_args(args: &[crate::lang::functions::Argument], types: &model::common::types::all::Pass) -> Vec<HashMap<&'static str, Value>> {
    args.iter()
        .filter_map(|arg| {
            let arg_type = types.get(arg.ty)?;
            let is_ref = matches!(&arg_type.kind, TypeKind::Pointer(p) if p.kind == PointerKind::ByRef);
            let decorated = match &arg_type.decorators.param {
                Some(d) => format!("{d} {}", arg_type.name),
                None => arg_type.name.clone(),
            };
            let mut m = make_arg(&arg.name, &decorated, is_ref);
            if arg.ty == crate::lang::types::csharp::CANCELLATION_TOKEN {
                m.insert("has_default", Value::String("true".into()));
                m.insert("default_value", Value::String("default".into()));
            } else {
                m.insert("has_default", Value::String("false".into()));
            }
            Some(m)
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
    as_ok: bool,
    method_name: &str,
    interop_name: &str,
    args: &[HashMap<&str, Value>],
    docs: &str,
    visibility: &str,
    self_arg: &str,
) -> Result<String, crate::Error> {
    let mut context = Context::new();
    context.insert("rval", rval);
    context.insert("is_void", &is_void);
    context.insert("as_ok", &as_ok);
    context.insert("method_name", &method_name);
    context.insert("interop_name", &interop_name);
    context.insert("args", args);
    context.insert("docs", docs);
    context.insert("visibility", visibility);
    context.insert("self_arg", self_arg);
    Ok(templates.render("rust/service/body_methods.cs", &context)?)
}

fn render_async(
    templates: &interoptopus_backends::template::TemplateEngine,
    task_rval: &str,
    method_name: &str,
    interop_name: &str,
    args: &[HashMap<&str, Value>],
    docs: &str,
    visibility: &str,
    self_arg: &str,
    trampoline_field: &str,
    is_task_void: bool,
) -> Result<String, crate::Error> {
    let mut context = Context::new();
    context.insert("task_rval", task_rval);
    context.insert("method_name", method_name);
    context.insert("interop_name", interop_name);
    context.insert("args", args);
    context.insert("docs", docs);
    context.insert("visibility", visibility);
    context.insert("self_arg", self_arg);
    context.insert("trampoline_field", trampoline_field);
    context.insert("is_task_void", &is_task_void);
    Ok(templates.render("rust/service/body_methods_async.cs", &context)?)
}

struct ResultRval {
    as_ok: bool,
    rval_name: Option<String>,
    is_void: bool,
}

fn resolve_result_rval(rval_kind: Option<&TypeKind>, types: &model::common::types::all::Pass) -> ResultRval {
    match rval_kind {
        Some(TypeKind::TypePattern(TypePattern::Result(ok_ty, _, _))) => {
            let ok_is_void = matches!(types.get(*ok_ty).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
            let ok_name = if ok_is_void {
                "void".to_string()
            } else {
                types.get(*ok_ty).map_or_else(|| "void".to_string(), |t| t.name.clone())
            };
            ResultRval { as_ok: true, rval_name: Some(ok_name), is_void: ok_is_void }
        }
        _ => ResultRval { as_ok: false, rval_name: None, is_void: false },
    }
}

/// Returns true if the function's first argument is a service type (not `IntPtr`).
fn has_service_self_arg(func: &crate::lang::functions::Function, types: &model::common::types::all::Pass) -> bool {
    func.signature
        .arguments
        .first()
        .and_then(|arg| types.get(arg.ty))
        .is_some_and(|ty| matches!(&ty.kind, TypeKind::Service))
}
