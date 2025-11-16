//! Builds a map of C# TypeId to type names (using Rust names for now).

use crate::model::TypeId;
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{ModelResult, model_id_maps};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    names: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { names: Default::default() }
    }

    pub fn process(&mut self, id_map: &model_id_maps::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            // Get the C# TypeId
            let Some(cs_id) = id_map.cs_from_rust(*rust_id) else {
                continue;
            };

            // Skip if we've already mapped this name
            if self.names.contains_key(&cs_id) {
                continue;
            }

            // For now, just use the Rust name
            self.names.insert(cs_id, ty.name.clone());
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn get_name(&self, ty: TypeId) -> Option<&String> {
        self.names.get(&ty)
    }
}
