//! Container for all C# functions (originals + overloads).
//!
//! This is the single source of truth for all functions. Output passes should
//! query this pass rather than separately querying originals or overload registries.

use crate::lang::FunctionId;
use crate::lang::functions::{Function, FunctionKind};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Config {}

#[derive(Debug)]
pub struct Pass {
    functions: BTreeMap<FunctionId, Function>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { functions: BTreeMap::default() }
    }

    pub fn register(&mut self, id: FunctionId, function: Function) {
        self.functions.insert(id, function);
    }

    #[must_use]
    pub fn get(&self, id: FunctionId) -> Option<&Function> {
        self.functions.get(&id)
    }

    pub fn get_mut(&mut self, id: FunctionId) -> Option<&mut Function> {
        self.functions.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.functions.iter()
    }

    /// Iterate only over original (non-overload) functions.
    pub fn originals(&self) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.functions.iter().filter(|(_, f)| matches!(f.kind, FunctionKind::Original))
    }

    /// Iterate only over overload functions.
    pub fn overloads(&self) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.functions.iter().filter(|(_, f)| matches!(f.kind, FunctionKind::Overload(_)))
    }

    /// Get all overload functions whose base is the given original function ID.
    pub fn overloads_for(&self, original_id: FunctionId) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.functions.iter().filter(move |(_, f)| match &f.kind {
            FunctionKind::Overload(o) => o.base == original_id,
            FunctionKind::Original => false,
        })
    }
}
