//! Maps Rust arrays to C# arrays.

use crate::lang::types::{Array, TypeKind};
use crate::model::TypeId;
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{ModelResult, model_id_maps, model_type_kinds};
use interoptopus::lang;

#[derive(Default)]
pub struct Config {}

pub struct Pass {}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, kinds: &mut model_type_kinds::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let rust_array = match &ty.kind {
                lang::types::TypeKind::Array(arr) => arr,
                _ => continue,
            };

            // Create C# TypeId for the array
            let cs_id = TypeId::from_id(rust_id.id());

            // Check if we already processed this array
            if id_map.cs_from_rust(*rust_id).is_some() {
                continue;
            }

            // Try to convert the element type
            let Some(cs_element_type) = id_map.cs_from_rust(rust_array.ty) else {
                continue;
            };

            // Create the C# array with mapped element type
            let cs_array = Array { ty: cs_element_type, len: rust_array.len };

            id_map.set_rust_to_cs(*rust_id, cs_id);
            kinds.set_kind(cs_id, TypeKind::Array(cs_array));
            outcome.changed();
        }

        Ok(outcome)
    }
}
