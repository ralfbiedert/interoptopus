//! Reflows function visibility based on signature types.
//!
//! A function's visibility is set to the most restrictive visibility among all
//! types in its signature (arguments + return type). If all types are public,
//! the function becomes public; if any type is internal, the function becomes
//! internal, and so on.

use crate::lang::meta::Visibility;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        fns_all: &mut model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect function ids first to avoid borrowing fns_all while mutating.
        let fn_ids: Vec<_> = fns_all.iter().map(|(&id, _)| id).collect();

        for fn_id in fn_ids {
            let Some(func) = fns_all.get(fn_id) else { continue };

            // Compute the most restrictive visibility across all signature types.
            let mut target = Visibility::Public;

            if let Some(rval_ty) = types.get(func.signature.rval) {
                target = target.most_restrictive(&rval_ty.visibility);
            }

            for arg in &func.signature.arguments {
                if let Some(arg_ty) = types.get(arg.ty) {
                    target = target.most_restrictive(&arg_ty.visibility);
                }
            }

            if func.visibility == target {
                continue;
            }

            let func = fns_all.get_mut(fn_id).unwrap();
            func.visibility = target;
            outcome.changed();
        }

        Ok(outcome)
    }
}
