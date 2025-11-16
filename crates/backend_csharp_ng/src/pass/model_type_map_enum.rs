//! Creates DataEnum types from computed enum variants.

use crate::lang::types::{DataEnum, TypeKind};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, model_id_maps, model_type_kinds, model_type_map_enum_variants};
use interoptopus::lang;

#[derive(Default)]
pub struct Config {}

pub struct Pass {}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(
        &mut self,
        id_map: &model_id_maps::Pass,
        kinds: &mut model_type_kinds::Pass,
        variants_pass: &model_type_map_enum_variants::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            match &ty.kind {
                lang::types::TypeKind::Enum(_) => {}
                _ => continue,
            }

            // Get the C# TypeId
            let Some(cs_id) = id_map.cs_from_rust(*rust_id) else {
                continue;
            };

            // Check if we've already processed this type
            if kinds.contains(&cs_id) {
                continue;
            }

            // Get the converted variants
            let Some(variants) = variants_pass.get_variants(cs_id) else {
                continue;
            };

            // Create the data enum
            let data_enum = DataEnum { variants: variants.clone() };

            kinds.set_kind(cs_id, TypeKind::DataEnum(data_enum));
            outcome.changed();
        }

        Ok(outcome)
    }
}
