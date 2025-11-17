//! Creates Composite types from computed struct fields and blittability.

use crate::lang::types::{Composite, TypeKind};
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{ModelResult, PassInfo, model_id_maps, model_type_kinds, model_type_map_struct_blittable, model_type_map_struct_fields};
use interoptopus::lang;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: "model_type_map_struct" },
        }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut super::PassMeta,
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
            let Some(cs_id) = id_map.cs_from_rust(*rust_id) else {
                // Type not yet mapped, skip
                pass_meta.lost_found.missing(self.info, super::MissingItem::RustType(*rust_id));
                outcome = Changed;
                continue;
            };

            // Check if we've already processed this type
            if kinds.contains(&cs_id) {
                continue;
            }

            // Get the converted fields
            let Some(fields) = fields_pass.get_fields(cs_id) else {
                pass_meta.lost_found.missing(self.info, super::MissingItem::CsType(cs_id));
                continue;
            };

            // Get the blittability
            let Some(kind) = blittable_pass.blittable(cs_id) else {
                pass_meta.lost_found.missing(self.info, super::MissingItem::CsType(cs_id));
                continue;
            };

            // Create the composite
            let composite = Composite { fields: fields.clone(), repr: rust_struct.repr, kind };

            kinds.set_kind(cs_id, TypeKind::Composite(composite));
            outcome.changed();
        }

        Ok(outcome)
    }
}
