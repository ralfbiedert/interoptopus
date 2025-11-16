//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::model::TypeId;
use crate::pass::ModelResult;
use crate::pass::Outcome::Unchanged;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

type RsToCs = HashMap<interoptopus::inventory::TypeId, TypeId>;
type CsToRs = HashMap<TypeId, interoptopus::inventory::TypeId>;

pub struct Pass {
    rs_to_cs: RsToCs,
    cs_to_rs: RsToCs,
}

impl Pass {}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { rs_to_cs: Default::default(), cs_to_rs: Default::default() }
    }

    pub fn process(&mut self) -> ModelResult {
        Ok(Unchanged)
    }

    pub(crate) fn set_rust_to_cs(&mut self, rust_id: interoptopus::inventory::TypeId, cs_id: TypeId) {
        self.rs_to_cs.insert(rust_id, cs_id);
    }

    pub(crate) fn get_cs_from_rust(&self, rust_id: interoptopus::inventory::TypeId) -> Option<TypeId> {
        self.rs_to_cs.get(&rust_id).copied()
    }
}
