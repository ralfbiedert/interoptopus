pub mod plugin;
pub mod service;

use crate::lang::functions::{Argument, Signature};
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::lang::TypeId;
use crate::pass::model::common::types::all::Pass as TypesAll;
use interoptopus::lang::types::TypeInfo;

/// If the last argument is `AsyncCallback<T>`, returns the inner `TypeId`.
fn async_callback_inner(args: &[Argument], types: &TypesAll) -> Option<TypeId> {
    let last = args.last()?;
    let ty = types.get(last.ty)?;
    match &ty.kind {
        TypeKind::TypePattern(TypePattern::AsyncCallback(inner)) => Some(*inner),
        _ => None,
    }
}

/// If `type_id` is a pointer whose target is a Service, return the service's `TypeId`.
fn resolve_ptr_to_service_id(type_id: TypeId, types: &TypesAll) -> Option<TypeId> {
    let ty = types.get(type_id)?;
    if let TypeKind::Pointer(p) = &ty.kind {
        let target = types.get(p.target)?;
        if matches!(&target.kind, TypeKind::Service) {
            return Some(p.target);
        }
    }
    None
}

/// Resolve a return `TypeId` through pointer-to-service patterns.
///
/// - If `rval_id` is pointer-to-service → returns the service `TypeId` (so the
///   interface can emit `TSelf`).
/// - Otherwise → returns `rval_id` unchanged. Result types wrapping a service
///   pointer already have the correct managed name from the naming pass.
fn resolve_rval_type(rval_id: TypeId, types: &TypesAll) -> TypeId {
    if let Some(svc_id) = resolve_ptr_to_service_id(rval_id, types) {
        return svc_id;
    }
    rval_id
}

/// Build a C# method's resolved info from function arguments and return type.
///
/// Returns `(Signature, resolved_rval_id, is_async, unwrapped_result_id)`:
/// - `Signature`: arguments with async callback stripped, rval set to the resolved type.
/// - `resolved_rval_id`: the managed return `TypeId` (service or original).
/// - `is_async`: true if the original function uses `AsyncCallback<T>`.
/// - `unwrapped_result_id`: if the return was `Result<T, ErrorXXX>`, holds the
///   original Result `TypeId` (and `resolved_rval_id` is the unwrapped `T`).
///
/// When `unwrap_error_id` is `Some`, Result types whose error matches that id
/// are transparently unwrapped so the user-facing signature returns `T`.
pub(super) fn resolve_method_info(args: &[Argument], rval: TypeId, types: &TypesAll, unwrap_error_id: Option<TypeId>) -> (Signature, TypeId, bool, Option<TypeId>) {
    let async_inner = async_callback_inner(args, types);
    let is_async = async_inner.is_some();

    // For async functions, the rval may carry an override (service-specific TypeId) set by the
    // proc macro. Use it when available (non-void); otherwise fall back to the callback inner type.
    let rval_is_void = matches!(types.get(rval).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    let raw_rval_id = if is_async && rval_is_void { async_inner.unwrap_or(rval) } else { rval };
    let mut resolved_rval_id = resolve_rval_type(raw_rval_id, types);

    // Unwrap Result<T, ErrorXXX> → T for user-facing signatures.
    let unwrapped_result_id = if let Some(err_id) = unwrap_error_id {
        if let Some(ty) = types.get(resolved_rval_id) {
            if let TypeKind::TypePattern(TypePattern::Result(ok, err, _)) = &ty.kind {
                if *err == err_id {
                    let original = resolved_rval_id;
                    resolved_rval_id = resolve_rval_type(*ok, types);
                    Some(original)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let is_void = matches!(types.get(resolved_rval_id).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    let effective_rval = if is_void && is_async { rval } else { resolved_rval_id };

    let arg_count = if is_async { args.len().saturating_sub(1) } else { args.len() };
    let arguments: Vec<Argument> = args.iter().take(arg_count).cloned().collect();

    (Signature { arguments, rval: effective_rval }, resolved_rval_id, is_async, unwrapped_result_id)
}
