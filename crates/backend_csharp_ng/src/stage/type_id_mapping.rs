//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::id::TypeId;
use crate::stage::output_director;
use interoptopus::inventory::Inventory;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Config {
    _hidden: PhantomData<()>,
}

pub struct Stage {
    rust_to_cs: HashMap<interoptopus::inventory::TypeId, TypeId>,
}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self { rust_to_cs: Default::default() }
    }

    pub fn process(&mut self, inventory: &Inventory) {}
}
