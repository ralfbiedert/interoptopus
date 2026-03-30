//! Produces simple overloads that replace `IntPtr` arguments with `ref` types.
//!
//! These overloads are purely for C# signature convenience and don't require
//! us to emit a function body — C# handles the marshalling natively.
//!
//! Uses the `overload::pointer` type pass to look up the `ByRef` sibling `TypeId`
//! for each eligible `IntPtr` argument. Registers produced overloads into the
//! central `overload::all` pass.

use crate::lang::FunctionId;
use crate::lang::functions::overload::{Overload, OverloadKind};
use crate::lang::functions::{Argument, Function, FunctionKind, Signature};
use crate::lang::types::OverloadFamily;
use crate::lang::types::kind::TypeKind;
use crate::pass::Outcome::Unchanged;
use crate::pass::model::rust::fns::overload::{IntPtrEligibility, derive_overload_id, intptr_eligibility, is_eligible_intptr};
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
        all: &mut model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        overloads: &model::rust::types::overload::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect originals first to avoid borrowing `all` mutably while iterating.
        // Sort by ID for deterministic iteration order.
        let mut originals: Vec<_> = all.originals().map(|(&id, f)| (id, f.clone())).collect();
        originals.sort_by_key(|(id, _)| *id);

        for &(original_id, ref original_fn) in &originals {
            if self.processed.contains(&original_id) {
                continue;
            }

            // Check if any argument is an eligible IntPtr.
            // If any argument's eligibility is still unknown (target type not yet resolved),
            // defer and retry on the next iteration rather than permanently rejecting.
            let has_any_unknown = original_fn
                .signature
                .arguments
                .iter()
                .any(|arg| matches!(intptr_eligibility(arg.ty, types), IntPtrEligibility::Unknown));
            if has_any_unknown {
                continue; // Defer — don't mark as processed
            }

            let has_any_eligible = original_fn.signature.arguments.iter().any(|arg| is_eligible_intptr(arg.ty, types));
            if !has_any_eligible {
                self.processed.insert(original_id);
                continue;
            }

            // Has eligible IntPtr args, but families aren't available yet — skip and retry
            let all_families_available = original_fn.signature.arguments.iter().all(|arg| {
                if is_eligible_intptr(arg.ty, types) {
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
                let new_ty = if is_eligible_intptr(arg.ty, types) {
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
            let overload_fn = Function {
                emission: original_fn.emission.clone(),
                name: original_fn.name.clone(),
                docs: original_fn.docs.clone(),
                signature: overload_signature,
                kind: FunctionKind::Overload(Overload { kind: OverloadKind::Simple, base: original_id }),
            };

            all.register(overload_id, overload_fn);
            self.overloads.insert(overload_id);
            self.processed.insert(original_id);
            outcome.changed();
        }

        Ok(outcome)
    }
}
