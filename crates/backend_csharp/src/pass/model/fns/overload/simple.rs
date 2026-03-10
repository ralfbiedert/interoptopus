//! Produces simple overloads that replace `IntPtr` arguments with `ref` types.
//!
//! These overloads are purely for C# signature convenience and don't require
//! us to emit a function body — C# handles the marshalling natively.
//!
//! Uses the `overload::pointer` type pass to look up the `ByRef` sibling TypeId
//! for each eligible `IntPtr` argument.

use crate::lang::function::{Argument, Function, Signature};
use crate::lang::types::{Pointer, TypeKind};
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
        Self { info: PassInfo { name: file!() }, overloads: Default::default(), original_to_overloads: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        originals: &model::fns::originals::Pass,
        all: &mut model::fns::all::Pass,
        type_kinds: &model::types::kind::Pass,
        pointer_overloads: &model::types::overload::pointer::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&original_id, original_fn) in originals.iter() {
            // Skip if we've already processed this original
            if self.original_to_overloads.contains_key(&original_id) {
                continue;
            }

            // Check if any argument is an IntPtr
            let has_any_intptr = original_fn.signature.arguments.iter().any(|arg| {
                matches!(type_kinds.get(arg.ty), Some(TypeKind::Pointer(Pointer::IntPtr(_, _))))
            });

            // No IntPtr args at all — permanently mark as no overloads needed
            if !has_any_intptr {
                self.original_to_overloads.insert(original_id, Vec::new());
                continue;
            }

            // Has IntPtr args, but families aren't available yet — skip and retry next iteration
            let all_families_available = original_fn.signature.arguments.iter().all(|arg| {
                match type_kinds.get(arg.ty) {
                    Some(TypeKind::Pointer(Pointer::IntPtr(_, _))) => pointer_overloads.family(arg.ty).is_some(),
                    _ => true,
                }
            });

            if !all_families_available {
                continue;
            }

            // Build the overload signature replacing IntPtr args with their ByRef siblings
            let mut overload_args = Vec::new();
            for arg in &original_fn.signature.arguments {
                let new_ty = if matches!(type_kinds.get(arg.ty), Some(TypeKind::Pointer(Pointer::IntPtr(_, _)))) {
                    pointer_overloads.family(arg.ty).map(|f| f.by_ref).unwrap_or(arg.ty)
                } else {
                    arg.ty
                };
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

    pub fn iter(&self) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.overloads.iter()
    }

    pub fn overloads_for(&self, original_id: FunctionId) -> Option<&[FunctionId]> {
        self.original_to_overloads.get(&original_id).map(|v| v.as_slice())
    }
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
