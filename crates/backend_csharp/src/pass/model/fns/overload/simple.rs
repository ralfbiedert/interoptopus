//! Produces simple overloads that replace `IntPtr` arguments with `ref` types.
//!
//! These overloads are purely for C# signature convenience and don't require
//! us to emit a function body — C# handles the marshalling natively.
//!
//! Uses the `overload::pointer` type pass to look up the `ByRef` sibling `TypeId`
//! for each eligible `IntPtr` argument. Registers produced overloads into the
//! central `overload::all` pass.

use crate::lang::functions::overload::OverloadKind;
use crate::lang::functions::{Argument, Function, Signature};
use crate::lang::types::kind::{Pointer, PointerKind, TypeKind};
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
    overloads: HashSet<FunctionId>,
    processed: HashSet<FunctionId>,
}

impl Pass {
    #[must_use] 
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, overloads: HashSet::default(), processed: HashSet::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        originals: &model::fns::originals::Pass,
        all: &mut model::fns::all::Pass,
        overload_all: &mut model::fns::overload::all::Pass,
        types: &model::types::all::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
        overloads: &model::types::overload::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&original_id, original_fn) in originals.iter() {
            if self.processed.contains(&original_id) {
                continue;
            }

            // Check if any argument is an eligible IntPtr (pointee is AsIs or To, not a class)
            let has_any_eligible = original_fn
                .signature
                .arguments
                .iter()
                .any(|arg| is_eligible_intptr(arg.ty, types, managed_conversion));

            if !has_any_eligible {
                self.processed.insert(original_id);
                continue;
            }

            // Has eligible IntPtr args, but families aren't available yet — skip and retry
            let all_families_available = original_fn.signature.arguments.iter().all(|arg| {
                if is_eligible_intptr(arg.ty, types, managed_conversion) {
                    matches!(overloads.get(arg.ty), Some(OverloadFamily::Pointer(_)))
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
                let new_ty = if is_eligible_intptr(arg.ty, types, managed_conversion) {
                    match overloads.get(arg.ty) {
                        Some(OverloadFamily::Pointer(f)) => f.by_ref,
                        _ => arg.ty,
                    }
                } else {
                    arg.ty
                };
                overload_args.push(Argument { name: arg.name.clone(), ty: new_ty });
            }

            let overload_signature = Signature { arguments: overload_args, rval: original_fn.signature.rval };
            let overload_id = derive_overload_id(original_id, &overload_signature);
            let overload_fn = Function { name: original_fn.name.clone(), signature: overload_signature };

            all.register(overload_id, overload_fn);
            overload_all.register(original_id, overload_id, OverloadKind::Simple);
            self.overloads.insert(overload_id);
            self.processed.insert(original_id);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn iter(&self) -> impl Iterator<Item = FunctionId> + '_ {
        self.overloads.iter().copied()
    }
}
