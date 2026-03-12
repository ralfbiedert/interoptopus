//! Produces async overloads for functions whose last argument is `AsyncCallback`.
//!
//! When a function's last argument has type `TypePattern::AsyncCallback(T)`, this
//! pass creates an overloaded function that:
//! - Removes the last (callback) argument
//! - Applies body-style arg transforms (ref, delegate wrap) to remaining args
//! - Sets `RvalTransform::AsyncTask(result_ty)` so output knows to render `Task<T>`
//!
//! The `T` in `AsyncCallback<T>` is the full `Result<Ok, Err>` type that the
//! callback will receive. Output passes use this to generate trampoline classes
//! and `Task<Ok>` returning overloads.

use crate::lang::functions::overload::{ArgTransform, FnTransforms, OverloadKind, RvalTransform};
use crate::lang::functions::{Argument, Function, Signature};
use crate::lang::types::kind::{DelegateKind, TypeKind, TypePattern};
use crate::lang::types::{ManagedConversion, OverloadFamily};
use crate::lang::{FunctionId, TypeId};
use crate::pass::model::fns::overload::{derive_overload_id, is_eligible_intptr};
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    /// Maps original function ID → async result TypeId. `None` = not async.
    async_result: HashMap<FunctionId, Option<TypeId>>,
    /// Set of unique async Result type IDs that need trampoline classes.
    trampoline_types: HashSet<TypeId>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, async_result: Default::default(), trampoline_types: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        originals: &model::fns::originals::Pass,
        fns_all: &mut model::fns::all::Pass,
        overload_all: &mut model::fns::overload::all::Pass,
        types: &model::types::all::Pass,
        overloads: &model::types::overload::all::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&original_id, original_fn) in originals.iter() {
            if self.async_result.contains_key(&original_id) {
                continue;
            }

            // Check if the last argument is an AsyncCallback
            let Some(last_arg) = original_fn.signature.arguments.last() else {
                self.async_result.insert(original_id, None);
                continue;
            };

            let Some(last_arg_ty) = types.get(last_arg.ty) else {
                continue; // Type not available yet, retry next cycle
            };

            let async_result_ty = match &last_arg_ty.kind {
                TypeKind::TypePattern(TypePattern::AsyncCallback(t)) => *t,
                _ => {
                    self.async_result.insert(original_id, None);
                    continue;
                }
            };

            // Verify the async result type is actually a Result
            let Some(result_ty) = types.get(async_result_ty) else {
                continue; // Type not available yet
            };
            if !matches!(&result_ty.kind, TypeKind::TypePattern(TypePattern::Result(_, _, _))) {
                self.async_result.insert(original_id, None);
                continue;
            }

            // All args except the last (callback) — check if type overload families are ready
            let non_callback_args = &original_fn.signature.arguments[..original_fn.signature.arguments.len() - 1];

            let all_ready = non_callback_args.iter().all(|arg| {
                if is_delegate_class(arg.ty, types) {
                    matches!(overloads.get(arg.ty), Some(OverloadFamily::Delegate(_)))
                } else if is_eligible_intptr(arg.ty, types, managed_conversion) {
                    matches!(overloads.get(arg.ty), Some(OverloadFamily::Pointer(_)))
                } else {
                    true
                }
            });

            if !all_ready {
                continue;
            }

            // Build per-argument transforms and overloaded signature (excluding callback)
            let mut arg_transforms = Vec::new();
            let mut overload_args = Vec::new();

            for arg in non_callback_args {
                if let Some(OverloadFamily::Delegate(family)) = is_delegate_class(arg.ty, types).then(|| overloads.get(arg.ty)).flatten() {
                    overload_args.push(Argument { name: arg.name.clone(), ty: family.signature });
                    arg_transforms.push(ArgTransform::WrapDelegate);
                } else if let Some(OverloadFamily::Pointer(family)) = is_eligible_intptr(arg.ty, types, managed_conversion).then(|| overloads.get(arg.ty)).flatten() {
                    overload_args.push(Argument { name: arg.name.clone(), ty: family.by_ref });
                    arg_transforms.push(ArgTransform::Ref);
                } else {
                    overload_args.push(Argument { name: arg.name.clone(), ty: arg.ty });
                    arg_transforms.push(ArgTransform::PassThrough);
                }
            }

            let overload_signature = Signature { arguments: overload_args, rval: async_result_ty };
            let overload_id = derive_overload_id(original_id, &overload_signature);
            let overload_fn = Function { name: original_fn.name.clone(), signature: overload_signature };

            let transforms = FnTransforms { rval: RvalTransform::AsyncTask(async_result_ty), args: arg_transforms };

            fns_all.register(overload_id, overload_fn);
            overload_all.register(original_id, overload_id, OverloadKind::Async(transforms));
            self.trampoline_types.insert(async_result_ty);
            self.async_result.insert(original_id, Some(async_result_ty));
            outcome.changed();
        }

        Ok(outcome)
    }

    /// Returns the async result TypeId for a given original function, if it is async.
    pub fn async_result_ty(&self, original_id: FunctionId) -> Option<TypeId> {
        self.async_result.get(&original_id).and_then(|o| *o)
    }

    /// Returns the set of unique async Result type IDs that need trampoline classes.
    pub fn trampoline_types(&self) -> &HashSet<TypeId> {
        &self.trampoline_types
    }
}

fn is_delegate_class(ty: TypeId, types: &model::types::all::Pass) -> bool {
    matches!(types.get(ty).map(|t| &t.kind), Some(TypeKind::Delegate(d)) if d.kind == DelegateKind::Class)
}
