//! Central registry of all function overloads (simple and body).
//!
//! Maps each original FunctionId to its overloaded FunctionIds. Both the simple
//! and body overload passes register their results here. Downstream passes
//! (output rendering, service methods) query this pass to discover all overloads
//! for a given function.

use crate::lang::FunctionId;
use crate::pass::PassInfo;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    overloads: HashMap<FunctionId, Vec<FunctionId>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, overloads: Default::default() }
    }

    /// Register an overload for an original function.
    pub fn register(&mut self, original_id: FunctionId, overload_id: FunctionId) {
        self.overloads.entry(original_id).or_default().push(overload_id);
    }

    /// Get all overload FunctionIds for an original function.
    pub fn overloads_for(&self, original_id: FunctionId) -> Option<&[FunctionId]> {
        self.overloads.get(&original_id).map(|v| v.as_slice())
    }
}
