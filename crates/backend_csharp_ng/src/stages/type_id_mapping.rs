//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::lang::types::TypeIdCs;
use crate::{StageData, stage_without_config};
use interoptopus::inventory::{Inventory, TypeId};
use std::collections::HashMap;

stage_without_config!();

#[derive(Default)]
pub struct Data {
    /// Maps a Rust `TypeId` to a C# `CsTypeId`
    pub rust_to_cs: HashMap<TypeId, TypeIdCs>,
}

impl Stage {
    fn process(&mut self, data: &mut StageData, inventory: &Inventory) {
        for key in inventory.types.keys() {
            let cs = TypeIdCs::from_id(key.id());
            data.type_id_mapping.rust_to_cs.insert(*key, cs);
        }
    }
}
