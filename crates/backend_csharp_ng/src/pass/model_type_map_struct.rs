//! Creates Composite types from computed struct fields and blittability.

use crate::lang::types::{Composite, TypeKind};
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{ModelResult, model_id_maps, model_type_kinds, model_type_map_struct_blittable, model_type_map_struct_fields};
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
        fields_pass: &model_type_map_struct_fields::Pass,
        blittable_pass: &model_type_map_struct_blittable::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let rust_struct = match &ty.kind {
                lang::types::TypeKind::Struct(x) => x,
                _ => continue,
            };

            // Get the C# TypeId
            let Some(cs_id) = id_map.get_cs_from_rust(*rust_id) else {
                // Type not yet mapped, skip
                outcome = Changed;
                continue;
            };

            // Check if we've already processed this type
            if kinds.contains(&cs_id) {
                continue;
            }

            // Get the converted fields
            let Some(fields) = fields_pass.get_fields(cs_id) else {
                // Fields not yet available, skip
                outcome = Changed;
                continue;
            };

            // Get the blittability
            let Some(kind) = blittable_pass.blittable(cs_id) else {
                // Blittability not yet determined, skip
                outcome = Changed;
                continue;
            };

            // Create the composite
            let composite = Composite {
                fields: fields.clone(),
                repr: rust_struct.repr,
                kind,
            };

            kinds.set_kind(cs_id, TypeKind::Composite(composite));
            outcome = Changed;
        }

        Ok(outcome)
    }
}
