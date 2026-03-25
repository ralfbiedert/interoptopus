pub mod plugin;
pub mod service;

use crate::lang::TypeId;
use crate::lang::functions::{Argument, Signature};
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::pass::model;
use crate::pass::model::common::types::all::Pass as TypesAll;
use std::collections::HashMap;

/// If the last argument is `AsyncCallback<T>`, returns the inner `TypeId`.
pub(super) fn async_callback_inner(args: &[Argument], types: &TypesAll) -> Option<TypeId> {
    let last = args.last()?;
    let ty = types.get(last.ty)?;
    match &ty.kind {
        TypeKind::TypePattern(TypePattern::AsyncCallback(inner)) => Some(*inner),
        _ => None,
    }
}

/// Returns `"Task"` for void inner types or `"Task<TypeName>"` for value types.
/// Resolves pointer-to-service to the service class name.
pub(super) fn task_type_name(inner_id: TypeId, types: &TypesAll) -> Option<String> {
    let is_void = matches!(types.get(inner_id).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    if is_void {
        Some("Task".to_string())
    } else if let Some(svc_name) = resolve_ptr_to_service(inner_id, types) {
        Some(format!("Task<{svc_name}>"))
    } else {
        let name = &types.get(inner_id)?.name;
        Some(format!("Task<{name}>"))
    }
}

/// Builds a C# `Signature` from a function's arguments and return type,
/// handling async functions (replacing `AsyncCallback<T>` return with `Task<T>`
/// and stripping the callback parameter).
///
/// Pointer-to-service types in return position are resolved to the service class name.
pub(super) fn csharp_signature(args: &[Argument], rval: TypeId, types: &TypesAll) -> Option<(Signature, String)> {
    let async_inner = async_callback_inner(args, types);

    let rval_name = if let Some(inner_id) = async_inner {
        task_type_name(inner_id, types)?
    } else if let Some(svc_name) = resolve_ptr_to_service(rval, types) {
        svc_name
    } else {
        types.get(rval)?.name.clone()
    };

    let arg_count = if async_inner.is_some() { args.len().saturating_sub(1) } else { args.len() };
    let arguments: Vec<Argument> = args.iter().take(arg_count).cloned().collect();
    let rval_type_id = if let Some(inner_id) = async_inner { inner_id } else { rval };

    Some((Signature { arguments, rval: rval_type_id }, rval_name))
}

/// If `type_id` is a pointer whose target is a Service, return the service class name.
fn resolve_ptr_to_service(type_id: TypeId, types: &TypesAll) -> Option<String> {
    let ty = types.get(type_id)?;
    if let TypeKind::Pointer(p) = &ty.kind {
        let target = types.get(p.target)?;
        if matches!(&target.kind, TypeKind::Service) {
            return Some(target.name.clone());
        }
    }
    None
}

/// Resolve the rval for an interface method.
///
/// If the rval is a Result wrapping a service handle (detected by method name convention),
/// returns the managed sibling type name (e.g., `ResultNestedBError`).
/// `method_name` is used to find the base method and determine which service is involved.
pub(super) fn resolve_interface_rval(
    rval_name: &str,
    method_name: &str,
    siblings: &model::dotnet::service_type_siblings::Pass,
    service_return_map: &HashMap<String, (String, TypeId)>,
    types: &TypesAll,
) -> String {
    // Try Result-containing suffixes first — these use the sibling Result type.
    for suffix in &["ResultAsync", "AsyncResult", "Result"] {
        if let Some(base) = method_name.strip_suffix(suffix) {
            if let Some((_svc_name, svc_type_id)) = service_return_map.get(base) {
                if let Some(svc_siblings) = siblings.for_service(*svc_type_id) {
                    if let Some(result_sibling_id) = svc_siblings.result {
                        if let Some(sibling_ty) = types.get(result_sibling_id) {
                            if suffix.contains("Async") {
                                return format!("Task<{}>", sibling_ty.name);
                            }
                            return sibling_ty.name.clone();
                        }
                    }
                }
            }
        }
    }

    // Pure async suffix — use the service class name directly in Task<>.
    if let Some(base) = method_name.strip_suffix("Async") {
        if let Some((svc_name, _)) = service_return_map.get(base) {
            return format!("Task<{svc_name}>");
        }
    }
    rval_name.to_string()
}
