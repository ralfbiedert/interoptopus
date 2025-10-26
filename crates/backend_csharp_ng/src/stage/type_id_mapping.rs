//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::id::TypeId;
use interoptopus::inventory::Inventory;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Stage {
    rust_to_cs: HashMap<interoptopus::inventory::TypeId, TypeId>,
}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self { rust_to_cs: Default::default() }
    }

    pub fn process(&mut self, inventory: &Inventory) {}
}
