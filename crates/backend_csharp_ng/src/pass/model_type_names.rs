//! Builds a map of C# TypeId to type names (using Rust names for now).

use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model_id_maps, ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    names: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "model_type_names" }, names: Default::default() }
    }

    pub fn process(&mut self, pass_meta: &mut super::PassMeta, id_map: &model_id_maps::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, super::MissingItem::RustType(*rust_id));

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
