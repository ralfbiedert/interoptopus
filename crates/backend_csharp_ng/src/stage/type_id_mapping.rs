//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::lang::types::TypeIdCs;
use interoptopus::inventory::{Inventory, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Stage {
    rust_to_cs: HashMap<TypeId, TypeIdCs>,
}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self { rust_to_cs: Default::default() }
    }

    pub fn process(&mut self, inventory: &Inventory) {
        for key in inventory.types.keys() {
            let cs = TypeIdCs::from_id(key.id());
            self.rust_to_cs.insert(*key, cs);
        }
    }
}
