//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, OutputResult};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

type RsToCs = HashMap<interoptopus::inventory::TypeId, TypeId>;
type CsToRs = HashMap<TypeId, interoptopus::inventory::TypeId>;

pub struct Pass {
    rs_to_cs: RsToCs,
    cs_to_rs: RsToCs,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { rs_to_cs: Default::default(), cs_to_rs: Default::default() }
    }

    pub fn process(&mut self, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        Ok(Unchanged)
    }
}
