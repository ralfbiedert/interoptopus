//! Computes per-argument and per-rval transforms for body overloads.
//!
//! When a function has delegate class arguments, it is eligible for a body overload.
//! This pass determines the transform kind for each argument and the return value,
//! storing them keyed by the original function ID. Output passes use these transforms
//! together with the originals and type overload passes to render the actual overload.

use crate::lang::overload::{ArgTransform, RvalTransform};
use crate::lang::types::{DelegateKind, ManagedConversion, Pointer, PointerKind, TypeKind};
use crate::lang::{FunctionId, TypeId};
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

/// Per-function body overload transforms.
#[derive(Clone, Debug)]
pub struct FnTransforms {
    pub rval: RvalTransform,
    pub args: Vec<ArgTransform>,
}

pub struct Pass {
    info: PassInfo,
    /// Maps original function ID to its body overload transforms. `None` means no body overload.
    transforms: HashMap<FunctionId, Option<FnTransforms>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: file!() },
            transforms: Default::default(),
        }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        originals: &model::fns::originals::Pass,
        type_kinds: &model::types::kind::Pass,
        pointer_overloads: &model::types::overload::pointer::Pass,
        delegate_overloads: &model::types::overload::delegate::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&original_id, original_fn) in originals.iter() {
            if self.transforms.contains_key(&original_id) {
                continue;
            }

            // Check if any argument is a delegate class (eligible for body overload)
            let has_any_delegate = original_fn.signature.arguments.iter().any(|arg| is_delegate_class(arg.ty, type_kinds));

            if !has_any_delegate {
                self.transforms.insert(original_id, None);
                continue;
            }

            // Check that all required sibling types are available
            let all_ready = original_fn.signature.arguments.iter().all(|arg| {
                if is_delegate_class(arg.ty, type_kinds) {
                    delegate_overloads.family(arg.ty).is_some()
                } else if is_eligible_intptr(arg.ty, type_kinds, managed_conversion) {
                    pointer_overloads.family(arg.ty).is_some()
                } else {
                    true
                }
            });

            if !all_ready {
                continue;
            }

            // Compute per-argument transforms
            let args = original_fn
                .signature
                .arguments
                .iter()
                .map(|arg| {
                    if is_delegate_class(arg.ty, type_kinds) {
                        ArgTransform::WrapDelegate
                    } else if is_eligible_intptr(arg.ty, type_kinds, managed_conversion) {
                        ArgTransform::Ref
                    } else {
                        ArgTransform::PassThrough
                    }
                })
                .collect();

            self.transforms.insert(original_id, Some(FnTransforms { rval: RvalTransform::PassThrough, args }));
            outcome.changed();
        }

        Ok(outcome)
    }

    /// Returns the body overload transforms for a function, if it has one.
    pub fn transforms(&self, id: FunctionId) -> Option<&FnTransforms> {
        self.transforms.get(&id)?.as_ref()
    }

    /// Iterates over all functions that have body overloads.
    pub fn iter(&self) -> impl Iterator<Item = (FunctionId, &FnTransforms)> {
        self.transforms.iter().filter_map(|(&id, t)| t.as_ref().map(|t| (id, t)))
    }
}

fn is_delegate_class(ty: TypeId, type_kinds: &model::types::kind::Pass) -> bool {
    matches!(type_kinds.get(ty), Some(TypeKind::Delegate(d)) if d.kind == DelegateKind::Class)
}

fn is_eligible_intptr(ty: TypeId, type_kinds: &model::types::kind::Pass, managed_conversion: &model::types::info::managed_conversion::Pass) -> bool {
    let Some(TypeKind::Pointer(Pointer { kind: PointerKind::IntPtr(_), target })) = type_kinds.get(ty) else {
        return false;
    };
    matches!(managed_conversion.managed_conversion(*target), Some(ManagedConversion::AsIs | ManagedConversion::To))
}
