//! Creates DataEnum types from computed enum variants.

use crate::lang::types::{DataEnum, TypeKind};
use crate::pass::Outcome::{Changed, Unchanged};
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
            let Some(cs_id) = id_map.get_cs_from_rust(*rust_id) else {
                // Type not yet mapped, skip
                outcome = Changed;
                continue;
            };

            // Check if we've already created the data enum
            if matches!(kinds.iter().find(|(id, _)| **id == cs_id).map(|(_, k)| k), Some(TypeKind::DataEnum(_))) {
                continue;
            }

            // Get the converted variants
            let Some(variants) = variants_pass.get_variants(cs_id) else {
                // Variants not yet available, skip
                outcome = Changed;
                continue;
            };

            // Create the data enum
            let data_enum = DataEnum {
                variants: variants.clone(),
            };

            kinds.set_kind(cs_id, TypeKind::DataEnum(data_enum));
            outcome = Changed;
        }

        Ok(outcome)
    }
}
