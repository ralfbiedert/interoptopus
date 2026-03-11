//! Produces body overloads that replace delegate class and IntPtr arguments.
//!
//! When a function has delegate class arguments, this pass creates an overloaded
//! Function where delegates become their signature type, IntPtrs become ref types,
//! and registers it in both `fn_all` and `overload::all`.
//!
//! The output pass for body overloads renders the wrapping/disposal logic using
//! the per-argument transforms stored here.

use crate::lang::functions::overload::{ArgTransform, RvalTransform};
use crate::lang::functions::{Argument, Function, Signature};
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
        all: &mut model::fns::all::Pass,
        overload_all: &mut model::fns::overload::all::Pass,
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

            // Build per-argument transforms and the overloaded signature
            let mut arg_transforms = Vec::new();
            let mut overload_args = Vec::new();

            for arg in &original_fn.signature.arguments {
                if is_delegate_class(arg.ty, type_kinds) {
                    let family = delegate_overloads.family(arg.ty).unwrap();
                    overload_args.push(Argument { name: arg.name.clone(), ty: family.signature });
                    arg_transforms.push(ArgTransform::WrapDelegate);
                } else if is_eligible_intptr(arg.ty, type_kinds, managed_conversion) {
                    let family = pointer_overloads.family(arg.ty).unwrap();
                    overload_args.push(Argument { name: arg.name.clone(), ty: family.by_ref });
                    arg_transforms.push(ArgTransform::Ref);
                } else {
                    overload_args.push(Argument { name: arg.name.clone(), ty: arg.ty });
                    arg_transforms.push(ArgTransform::PassThrough);
                }
            }

            let overload_signature = Signature { arguments: overload_args, rval: original_fn.signature.rval };
            let overload_id = derive_overload_id(original_id, &overload_signature);
            let overload_fn = Function { name: original_fn.name.clone(), signature: overload_signature };

            all.register(overload_id, overload_fn);
            overload_all.register(original_id, overload_id);
            self.transforms.insert(original_id, Some(FnTransforms { rval: RvalTransform::PassThrough, args: arg_transforms }));
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

fn derive_overload_id(original_id: FunctionId, signature: &Signature) -> FunctionId {
    let mut id = FunctionId::from_id(original_id.id().derive_id(signature.rval.id()));
    for arg in &signature.arguments {
        id = FunctionId::from_id(id.id().derive_id(arg.ty.id()));
    }
    id
}
