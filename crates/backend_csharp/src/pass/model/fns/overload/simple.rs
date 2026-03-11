//! Produces simple overloads that replace `IntPtr` arguments with `ref` types.
//!
//! These overloads are purely for C# signature convenience and don't require
//! us to emit a function body — C# handles the marshalling natively.
//!
//! Uses the `overload::pointer` type pass to look up the `ByRef` sibling TypeId
//! for each eligible `IntPtr` argument. Registers produced overloads into the
//! central `overload::all` pass.

use crate::lang::functions::{Argument, Function, Signature};
use crate::lang::types::{ManagedConversion, Pointer, PointerKind, TypeKind};
use crate::lang::{FunctionId, TypeId};
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashSet;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    overloads: HashSet<FunctionId>,
    processed: HashSet<FunctionId>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, overloads: Default::default(), processed: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        originals: &model::fns::originals::Pass,
        all: &mut model::fns::all::Pass,
        overload_all: &mut model::fns::overload::all::Pass,
        type_kinds: &model::types::kind::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
        pointer_overloads: &model::types::overload::pointer::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&original_id, original_fn) in originals.iter() {
            if self.processed.contains(&original_id) {
                continue;
            }

            // Check if any argument is an eligible IntPtr (pointee is AsIs or To, not a class)
            let has_any_eligible = original_fn.signature.arguments.iter().any(|arg| {
                is_eligible_intptr(arg.ty, type_kinds, managed_conversion)
            });

            if !has_any_eligible {
                self.processed.insert(original_id);
                continue;
            }

            // Has eligible IntPtr args, but families aren't available yet — skip and retry
            let all_families_available = original_fn.signature.arguments.iter().all(|arg| {
                if is_eligible_intptr(arg.ty, type_kinds, managed_conversion) {
                    pointer_overloads.family(arg.ty).is_some()
                } else {
                    true
                }
            });

            if !all_families_available {
                continue;
            }

            // Build the overload signature replacing eligible IntPtr args with their ByRef siblings
            let mut overload_args = Vec::new();
            for arg in &original_fn.signature.arguments {
                let new_ty = if is_eligible_intptr(arg.ty, type_kinds, managed_conversion) {
                    pointer_overloads.family(arg.ty).map(|f| f.by_ref).unwrap_or(arg.ty)
                } else {
                    arg.ty
                };
                overload_args.push(Argument { name: arg.name.clone(), ty: new_ty });
            }

            let overload_signature = Signature { arguments: overload_args, rval: original_fn.signature.rval };
            let overload_id = derive_overload_id(original_id, &overload_signature);
            let overload_fn = Function { name: original_fn.name.clone(), signature: overload_signature };

            all.register(overload_id, overload_fn);
            overload_all.register(original_id, overload_id);
            self.overloads.insert(overload_id);
            self.processed.insert(original_id);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn is_overload(&self, id: FunctionId) -> bool {
        self.overloads.contains(&id)
    }

    pub fn iter_overloads(&self) -> impl Iterator<Item = FunctionId> + '_ {
        self.overloads.iter().copied()
    }
}

fn is_eligible_intptr(
    ty: TypeId,
    type_kinds: &model::types::kind::Pass,
    managed_conversion: &model::types::info::managed_conversion::Pass,
) -> bool {
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
