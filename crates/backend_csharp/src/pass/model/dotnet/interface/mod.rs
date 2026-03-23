pub mod plugin;
pub mod service;

use crate::lang::TypeId;
use crate::lang::functions::{Argument, Signature};
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::pass::model::common::types::all::Pass as TypesAll;

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
/// Returns `None` if the inner type is not yet resolved.
pub(super) fn task_type_name(inner_id: TypeId, types: &TypesAll) -> Option<String> {
    let is_void = matches!(types.get(inner_id).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    if is_void {
        Some("Task".to_string())
    } else {
        let name = &types.get(inner_id)?.name;
        Some(format!("Task<{name}>"))
    }
}

/// Builds a C# `Signature` from a function's arguments and return type,
/// handling async functions (replacing `AsyncCallback<T>` return with `Task<T>`
/// and stripping the callback parameter).
///
/// Returns `None` if any referenced type is not yet resolved in the type map.
pub(super) fn csharp_signature(args: &[Argument], rval: TypeId, types: &TypesAll) -> Option<(Signature, String)> {
    let async_inner = async_callback_inner(args, types);

    let rval_name = if let Some(inner_id) = async_inner {
        task_type_name(inner_id, types)?
    } else {
        let ty = types.get(rval)?;
        ty.name.clone()
    };

    let arg_count = if async_inner.is_some() { args.len().saturating_sub(1) } else { args.len() };
    let arguments: Vec<Argument> = args.iter().take(arg_count).cloned().collect();
    let rval_type_id = if let Some(inner_id) = async_inner { inner_id } else { rval };

    Some((Signature { arguments, rval: rval_type_id }, rval_name))
}
