pub mod all;
pub mod raw;
pub mod service;

use crate::lang::TypeId;
use crate::lang::functions::{Argument, Function};
use crate::lang::types::kind::Primitive;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::pass::{model, output};

/// Returns `(args_str, forward_str)` for a `[UnmanagedCallersOnly]` signature.
///
/// - `args_str`: unmanaged parameter types (e.g. `MyCallback.Unmanaged res`)
/// - `forward_str`: forwarded expressions with to-managed conversions (e.g. `res.IntoManaged()`)
pub(super) fn unmanaged_args(
    func: &Function,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let args: Vec<String> = func
        .signature
        .arguments
        .iter()
        .filter_map(|arg| {
            let ty_name = unmanaged_names.name(arg.ty)?;
            Some(format!("{ty_name} {}", arg.name))
        })
        .collect();

    let forward: Vec<String> = func
        .signature
        .arguments
        .iter()
        .map(|a| format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty)))
        .collect();

    (args.join(", "), forward.join(", "))
}

/// Returns `(args_str, forward_str)` for an async `[UnmanagedCallersOnly]` signature.
///
/// All args appear in `args_str` (the callback is blittable/AsIs). Only the
/// non-callback args appear in `forward_str`.
pub(super) fn unmanaged_args_except_last(
    func: &Function,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let n = func.signature.arguments.len().saturating_sub(1);

    let args: Vec<String> = func
        .signature
        .arguments
        .iter()
        .filter_map(|arg| {
            let ty_name = unmanaged_names.name(arg.ty)?;
            Some(format!("{ty_name} {}", arg.name))
        })
        .collect();

    let forward: Vec<String> = func
        .signature
        .arguments
        .iter()
        .take(n)
        .map(|a| format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty)))
        .collect();

    (args.join(", "), forward.join(", "))
}

/// Returns the unmanaged return type name, or `rval_type` unchanged for `AsIs`/void.
pub(super) fn rval_unmanaged_name<'a>(func: &Function, rval_type: &'a str, unmanaged_names: &'a output::common::conversion::unmanaged_names::Pass) -> &'a str {
    unmanaged_names.name(func.signature.rval).map_or(rval_type, String::as_str)
}

/// If the last argument is `AsyncCallback<T>`, returns the inner `TypeId`.
pub(super) fn async_callback_inner(func: &Function, types: &model::common::types::all::Pass) -> Option<TypeId> {
    let last = func.signature.arguments.last()?;
    async_callback_inner_from_args(std::slice::from_ref(last), types)
}

/// If the last element of `args` is `AsyncCallback<T>`, returns the inner `TypeId`.
fn async_callback_inner_from_args(args: &[Argument], types: &model::common::types::all::Pass) -> Option<TypeId> {
    let last = args.last()?;
    let ty = types.get(last.ty)?;
    match &ty.kind {
        TypeKind::TypePattern(TypePattern::AsyncCallback(inner)) => Some(*inner),
        _ => None,
    }
}

/// Returns the `.ContinueWith(...)` expression that invokes `cb.UnsafeComplete` after the task.
pub(super) fn async_continuation(
    inner_id: TypeId,
    types: &model::common::types::all::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> String {
    let is_void = matches!(types.get(inner_id).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    if is_void {
        "ContinueWith(_ => cb.UnsafeComplete())".to_string()
    } else {
        let suffix = unmanaged_conversion.to_unmanaged_suffix(inner_id);
        format!("ContinueWith(t => cb.UnsafeComplete(t.Result{suffix}))")
    }
}
