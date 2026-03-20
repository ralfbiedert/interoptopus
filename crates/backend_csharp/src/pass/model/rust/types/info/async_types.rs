//! Collects the set of unique async Result type IDs that need trampoline classes.
//!
//! Scans `fns::all` for async overloads and extracts the `AsyncTask`
//! result `TypeId` from each. The output trampoline pass uses this to know which
//! trampoline classes to generate.

use crate::lang::TypeId;
use crate::lang::functions::FunctionKind;
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

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, fns_all: &model::common::fns::all::Pass) -> ModelResult {
        let mut outcome = Unchanged;

        for (_, func) in fns_all.overloads() {
            if let FunctionKind::Overload(overload) = &func.kind
                && let OverloadKind::Async(transforms) = &overload.kind
                && let RvalTransform::AsyncTask(result_ty_id) = transforms.rval
                && self.trampoline_types.insert(result_ty_id)
            {
                outcome.changed();
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
