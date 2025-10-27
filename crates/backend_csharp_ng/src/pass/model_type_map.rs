//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::model::TypeId;
use crate::pass::ProcessError;
use interoptopus::inventory::RustInventory;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    rust_to_cs: HashMap<interoptopus::inventory::TypeId, TypeId>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { rust_to_cs: Default::default() }
    }

    pub fn process(&mut self, _: &RustInventory) -> ProcessError {
        // TODO ...
        Ok(())
    }
}
