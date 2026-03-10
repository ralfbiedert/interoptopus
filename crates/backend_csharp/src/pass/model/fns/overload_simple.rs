//! Produces simple overloads that replace `IntPtr` arguments with `ref` types.
//!
//! These overloads are purely for C# signature convenience and don't require
//! us to emit a function body — C# handles the marshalling natively.

use crate::lang::function::{Argument, Function, Signature};
use crate::lang::types::{ManagedConversion, Pointer, TypeKind};
use crate::lang::FunctionId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    overloads: HashMap<FunctionId, Function>,
    original_to_overloads: HashMap<FunctionId, Vec<FunctionId>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: file!() },
            overloads: Default::default(),
            original_to_overloads: Default::default(),
        }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        originals: &model::fns::originals::Pass,
        all: &mut model::fns::all::Pass,
        type_kinds: &model::types::kind::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&original_id, original_fn) in originals.iter() {
            // Skip if we've already processed this original
            if self.original_to_overloads.contains_key(&original_id) {
                continue;
            }

            // Check if any argument is an IntPtr pointing to an AsIs type
            let mut has_eligible_intptr = false;
            for arg in &original_fn.signature.arguments {
                if is_eligible_intptr(arg.ty, type_kinds, managed_conversion) {
                    has_eligible_intptr = true;
                    break;
                }
            }

            if !has_eligible_intptr {
                self.original_to_overloads.insert(original_id, Vec::new());
                continue;
            }

            // Build the overload signature with IntPtr replaced by Ref
            let mut overload_args = Vec::new();
            for arg in &original_fn.signature.arguments {
                let new_ty = intptr_to_ref(arg.ty, type_kinds, managed_conversion);
                overload_args.push(Argument { name: arg.name.clone(), ty: new_ty });
            }

            let overload_signature = Signature { arguments: overload_args, rval: original_fn.signature.rval };

            // Derive a new FunctionId by mixing the original with all signature type IDs
            let overload_id = derive_overload_id(original_id, &overload_signature);

            let overload_fn = Function { name: original_fn.name.clone(), signature: overload_signature };

            all.register(overload_id, overload_fn.clone());
            self.overloads.insert(overload_id, overload_fn);
            self.original_to_overloads.insert(original_id, vec![overload_id]);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn get(&self, id: FunctionId) -> Option<&Function> {
        self.overloads.get(&id)
    }

    pub fn overloads_for(&self, original_id: FunctionId) -> Option<&[FunctionId]> {
        self.original_to_overloads.get(&original_id).map(|v| v.as_slice())
    }
}

/// Returns `true` if the type is a `Pointer::IntPtr` pointing to an `AsIs` type.
fn is_eligible_intptr(
    ty: crate::lang::TypeId,
    type_kinds: &model::types::kind::Pass,
    managed_conversion: &model::types::info::managed_conversion::Pass,
) -> bool {
    let Some(TypeKind::Pointer(Pointer::IntPtr(pointee, _))) = type_kinds.get(ty) else {
        return false;
    };
    matches!(managed_conversion.managed_conversion(*pointee), Some(ManagedConversion::AsIs))
}

/// If the type is an eligible `IntPtr`, return the corresponding `Ref` type ID.
/// Otherwise return the type unchanged.
///
/// This looks up whether a `Ref` variant already exists in the type kinds pass.
/// For now we reuse the same TypeId since `Ref(pointee)` should already be mapped.
fn intptr_to_ref(
    ty: crate::lang::TypeId,
    type_kinds: &model::types::kind::Pass,
    managed_conversion: &model::types::info::managed_conversion::Pass,
) -> crate::lang::TypeId {
    if !is_eligible_intptr(ty, type_kinds, managed_conversion) {
        return ty;
    }

    // TODO: We need the Ref variant's TypeId. For now we return the same type
    // since the actual Ref TypeId mapping depends on how pointer types are registered.
    // This will need to be resolved once we wire up the pointer type registration.
    ty
}

/// Derives a unique `FunctionId` for the overload by mixing the original ID with
/// all signature type IDs.
fn derive_overload_id(original_id: FunctionId, signature: &Signature) -> FunctionId {
    let mut id = FunctionId::from_id(original_id.id().derive_id(signature.rval.id()));
    for arg in &signature.arguments {
        id = FunctionId::from_id(id.id().derive_id(arg.ty.id()));
    }
    id
}
