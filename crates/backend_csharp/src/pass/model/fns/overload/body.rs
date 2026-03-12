//! Produces body and async overloads for functions with delegate, `IntPtr`, or
//! `AsyncCallback` arguments.
//!
//! For each original function this pass checks two conditions:
//! - **Body**: Has delegate class args (but no async callback) → produces an overload
//!   with delegate→signature, intptr→ref transforms, registered as `OverloadKind::Body`.
//! - **Async**: Last arg is `AsyncCallback<T>` → produces an overload that drops the
//!   callback, applies the same arg transforms to remaining args, and registers as
//!   `OverloadKind::Async` with `RvalTransform::AsyncTask`.
//!
//! When both conditions are true, only the Async overload is emitted. Emitting both
//! would create two C# methods with identical parameter signatures differing only in
//! return type, which C# does not allow.

use crate::lang::functions::overload::{ArgTransform, FnTransforms, OverloadKind, RvalTransform};
use crate::lang::functions::{Argument, Function, Signature};
use crate::lang::types::kind::{DelegateKind, TypeKind, TypePattern};
use crate::lang::types::{ManagedConversion, OverloadFamily};
use crate::lang::{FunctionId, TypeId};
use crate::pass::Outcome::Unchanged;
use crate::pass::model::fns::overload::{derive_overload_id, is_eligible_intptr};
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashSet;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    processed: HashSet<FunctionId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, processed: HashSet::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        originals: &model::fns::originals::Pass,
        fns_all: &mut model::fns::all::Pass,
        overload_all: &mut model::fns::overload::all::Pass,
        types: &model::types::all::Pass,
        type_overloads: &model::types::overload::all::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&original_id, original_fn) in originals.iter() {
            if self.processed.contains(&original_id) {
                continue;
            }

            let args = &original_fn.signature.arguments;

            // Detect async: last arg is AsyncCallback<Result<T,E>>
            let async_result_ty = detect_async_callback(args, types);

            // Determine which args to check for body transforms (exclude callback if async)
            let transformable_args = match async_result_ty {
                Some(_) => &args[..args.len() - 1],
                None => &args[..],
            };

            let has_delegate = transformable_args.iter().any(|a| is_delegate_class(a.ty, types));
            let has_intptr = transformable_args.iter().any(|a| is_eligible_intptr(a.ty, types, managed_conversion));

            // Nothing to do for this function
            if async_result_ty.is_none() && !has_delegate {
                self.processed.insert(original_id);
                continue;
            }

            // Wait for type overload families to be available
            if !all_families_ready(transformable_args, types, type_overloads, managed_conversion) {
                continue;
            }

            // Also verify async result type is a Result if present
            if let Some(result_ty_id) = async_result_ty {
                let Some(result_ty) = types.get(result_ty_id) else { continue };
                if !matches!(&result_ty.kind, TypeKind::TypePattern(TypePattern::Result(_, _, _))) {
                    self.processed.insert(original_id);
                    continue;
                }
            }

            // Compute arg transforms for the transformable args
            let (overload_args, arg_transforms) = build_arg_transforms(transformable_args, types, type_overloads, managed_conversion);

            // Produce Body overload if there are delegate args but NO async callback.
            // When async is present, the Async overload already applies the same arg
            // transforms; emitting both would create two methods differing only in
            // return type, which C# rejects.
            if has_delegate && async_result_ty.is_none() {
                let sig = Signature { arguments: overload_args.clone(), rval: original_fn.signature.rval };
                let id = derive_overload_id(original_id, &sig);
                let func = Function { name: original_fn.name.clone(), signature: sig };
                let transforms = FnTransforms { rval: RvalTransform::PassThrough, args: arg_transforms.clone() };
                fns_all.register(id, func);
                overload_all.register(original_id, id, OverloadKind::Body(transforms));
                outcome.changed();
            }

            // Produce Async overload if last arg is AsyncCallback
            if let Some(result_ty_id) = async_result_ty {
                let sig = Signature { arguments: overload_args, rval: result_ty_id };
                let id = derive_overload_id(original_id, &sig);
                let func = Function { name: original_fn.name.clone(), signature: sig };
                let transforms = FnTransforms { rval: RvalTransform::AsyncTask(result_ty_id), args: arg_transforms };
                fns_all.register(id, func);
                overload_all.register(original_id, id, OverloadKind::Async(transforms));
                outcome.changed();
            }

            self.processed.insert(original_id);
        }

        Ok(outcome)
    }
}

/// If the last arg is `AsyncCallback<T>` where T is a Result, returns the `TypeId` of T.
fn detect_async_callback(args: &[Argument], types: &model::types::all::Pass) -> Option<TypeId> {
    let last = args.last()?;
    let ty = types.get(last.ty)?;
    match &ty.kind {
        TypeKind::TypePattern(TypePattern::AsyncCallback(t)) => Some(*t),
        _ => None,
    }
}

fn all_families_ready(
    args: &[Argument],
    types: &model::types::all::Pass,
    type_overloads: &model::types::overload::all::Pass,
    managed_conversion: &model::types::info::managed_conversion::Pass,
) -> bool {
    args.iter().all(|arg| {
        if is_delegate_class(arg.ty, types) {
            matches!(type_overloads.get(arg.ty), Some(OverloadFamily::Delegate(_)))
        } else if is_eligible_intptr(arg.ty, types, managed_conversion) {
            matches!(type_overloads.get(arg.ty), Some(OverloadFamily::Pointer(_)))
        } else {
            true
        }
    })
}

fn build_arg_transforms(
    args: &[Argument],
    types: &model::types::all::Pass,
    type_overloads: &model::types::overload::all::Pass,
    managed_conversion: &model::types::info::managed_conversion::Pass,
) -> (Vec<Argument>, Vec<ArgTransform>) {
    let mut overload_args = Vec::new();
    let mut transforms = Vec::new();

    for arg in args {
        if let Some(OverloadFamily::Delegate(family)) = is_delegate_class(arg.ty, types).then(|| type_overloads.get(arg.ty)).flatten() {
            overload_args.push(Argument { name: arg.name.clone(), ty: family.signature });
            transforms.push(ArgTransform::WrapDelegate);
        } else if let Some(OverloadFamily::Pointer(family)) = is_eligible_intptr(arg.ty, types, managed_conversion).then(|| type_overloads.get(arg.ty)).flatten() {
            overload_args.push(Argument { name: arg.name.clone(), ty: family.by_ref });
            transforms.push(ArgTransform::Ref);
        } else {
            overload_args.push(Argument { name: arg.name.clone(), ty: arg.ty });
            transforms.push(ArgTransform::PassThrough);
        }
    }

    (overload_args, transforms)
}

fn is_delegate_class(ty: TypeId, types: &model::types::all::Pass) -> bool {
    matches!(types.get(ty).map(|t| &t.kind), Some(TypeKind::Delegate(d)) if d.kind == DelegateKind::Class)
}
