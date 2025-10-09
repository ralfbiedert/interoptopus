//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::{StageData, stage_without_config};
use interoptopus::inventory::{Inventory, TypeId};
use interoptopus::new_id;
use std::collections::HashMap;

new_id!(CsTypeId);
stage_without_config!();

#[derive(Default)]
pub struct Data {
    /// Maps a Rust `TypeId` to a C# `CsTypeId`
    pub rust_to_cs: HashMap<TypeId, CsTypeId>,
}

impl Stage {
    fn process(&mut self, data: &mut StageData, inventory: &Inventory) {
        for key in inventory.types.keys() {
            let cs = CsTypeId::from_id(key.id());
            data.type_id_mapping.rust_to_cs.insert(*key, cs);
        }
    }
}
