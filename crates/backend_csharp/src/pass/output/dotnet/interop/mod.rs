pub mod all;
pub mod raw;
pub mod service;

use crate::lang::FunctionId;
use crate::lang::TypeId;
use crate::lang::functions::{Argument, Function};
use crate::lang::plugin::interface::{Method, ResultKind};
use crate::lang::types::kind::Primitive;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::pass::{model, output};
use std::collections::HashMap;

/// Splits an interface's methods into two lookup tables keyed by `FunctionId`:
/// - `try_wraps`: methods whose return is `Result<T, ExceptionError>` (Try). Maps to the
///   peeled Result's C# name; trampoline wraps the call with `FromCall`.
/// - `passthrough_wraps`: methods whose return is a user-defined `Result<T, E>` kept on the
///   user-facing signature. Maps to the Result's C# name; trampoline wraps the call with
///   `FromCallResult` to fold uncaught exceptions into `Panic`.
///
/// The two tables are disjoint by construction (`ResultKind` variants are mutually exclusive).
pub(super) fn split_result_kinds<'a, I>(methods: I, types: &'a model::common::types::all::Pass) -> (HashMap<FunctionId, &'a str>, HashMap<FunctionId, &'a str>)
where
    I: IntoIterator<Item = &'a Method>,
{
    let mut try_wraps = HashMap::new();
    let mut passthrough_wraps = HashMap::new();
    for m in methods {
        match m.result {
            Some(ResultKind::Try(id)) => {
                if let Some(t) = types.get(id) {
                    try_wraps.insert(m.base, t.name.as_str());
                }
            }
            Some(ResultKind::Passthrough(id)) => {
                if let Some(t) = types.get(id) {
                    passthrough_wraps.insert(m.base, t.name.as_str());
                }
            }
            None => {}
        }
    }
    (try_wraps, passthrough_wraps)
}

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
///
/// On a faulted or cancelled Task we send `AsyncOutcome::Cancelled` over the wire so the
/// awaiting Rust future surfaces `Err(AsyncCancelled)` instead of hanging forever.
///
/// When `has_result_passthrough` is true the caller wraps the awaited expression in
/// `FromCallResultAsync`, which converts faults to `Result::Panic` internally. The
/// continuation then only needs to watch for cancellation; faults reach this point as
/// normal completions carrying a `Panic` payload.
pub(super) fn async_continuation(
    inner_id: TypeId,
    types: &model::common::types::all::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
    has_result_passthrough: bool,
) -> String {
    let is_void = matches!(types.get(inner_id).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    let success = if is_void {
        "cb.UnsafeComplete()".to_string()
    } else {
        let suffix = unmanaged_conversion.to_unmanaged_suffix(inner_id);
        format!("cb.UnsafeComplete(t.Result{suffix})")
    };
    if has_result_passthrough {
        format!("ContinueWith(t => {{ if (t.IsCanceled) cb.UnsafeCompleteCancelled(); else {success}; }})")
    } else {
        format!("ContinueWith(t => {{ if (t.IsCanceled || t.IsFaulted) cb.UnsafeCompleteCancelled(); else {success}; }})")
    }
}
