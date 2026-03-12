//! Collects the set of unique async Result type IDs that need trampoline classes.
//!
//! Scans `overload::all` for `Async` overloads and extracts the `AsyncTask`
//! result `TypeId` from each. The output trampoline pass uses this to know which
//! trampoline classes to generate.

use crate::lang::TypeId;
use crate::lang::functions::overload::{OverloadKind, RvalTransform};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashSet;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    trampoline_types: HashSet<TypeId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, trampoline_types: HashSet::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        originals: &model::fns::originals::Pass,
        overload_all: &model::fns::overload::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&fn_id, _) in originals.iter() {
            let Some(entries) = overload_all.overloads_for(fn_id) else { continue };

            for (_, kind) in entries {
                if let OverloadKind::Async(transforms) = kind
                    && let RvalTransform::AsyncTask(result_ty_id) = transforms.rval
                    && self.trampoline_types.insert(result_ty_id)
                {
                    outcome.changed();
                }
            }
        }

        Ok(outcome)
    }

    /// Returns the set of unique async Result type IDs that need trampoline classes.
    #[must_use]
    pub fn trampoline_types(&self) -> &HashSet<TypeId> {
        &self.trampoline_types
    }
}
