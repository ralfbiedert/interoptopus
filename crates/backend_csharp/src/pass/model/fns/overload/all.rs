//! Central registry of all function overloads (simple, body, and async).
//!
//! Maps each original `FunctionId` to its overloaded `FunctionIds` and their kind.
//! All overload passes (simple, body, async) register their results here.
//! Downstream passes (output rendering, service methods) query this pass to
//! discover all overloads for a given function and filter by kind.

use crate::lang::FunctionId;
use crate::lang::functions::overload::OverloadKind;
use crate::pass::PassInfo;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    original_to_overload: HashMap<FunctionId, Vec<(FunctionId, OverloadKind)>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, original_to_overload: HashMap::default() }
    }

    /// Register an overload for an original function with its kind.
    pub fn register(&mut self, original_id: FunctionId, overload_id: FunctionId, kind: OverloadKind) {
        self.original_to_overload.entry(original_id).or_default().push((overload_id, kind));
    }

    /// Get all overload entries for an original function.
    #[must_use]
    pub fn overloads_for(&self, original_id: FunctionId) -> Option<&[(FunctionId, OverloadKind)]> {
        self.original_to_overload.get(&original_id).map(std::vec::Vec::as_slice)
    }
}
