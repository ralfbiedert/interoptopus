//! Creates Composite types from computed struct fields and blittability.

use crate::lang::types::{Composite, TypeKind};
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{model_id_maps, model_type_kinds, model_type_map_struct_blittable, model_type_map_struct_fields, ModelResult, PassInfo};
use interoptopus::lang;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "model_type_map_struct" } }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut super::PassMeta,
        id_map: &model_id_maps::Pass,
        kinds: &mut model_type_kinds::Pass,
        fields: &model_type_map_struct_fields::Pass,
        blittable: &model_type_map_struct_blittable::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let rust_struct = match &ty.kind {
                lang::types::TypeKind::Struct(x) => x,
                _ => continue,
            };

            // Get the C# TypeId
            let Some(cs_id) = id_map.ty(*rust_id) else {
                // Type not yet mapped, skip
                pass_meta.lost_found.missing(self.info, super::MissingItem::RustType(*rust_id));
                outcome = Changed;
                continue;
            };

            // Check if we've already processed this type
            if kinds.contains(&cs_id) {
                continue;
            }

            let fields = try_resolve!(fields.get_fields(cs_id), pass_meta, self.info, super::MissingItem::CsType(cs_id));
            let kind = try_resolve!(blittable.blittable(cs_id), pass_meta, self.info, super::MissingItem::CsType(cs_id));

            // Create the composite
            let composite = Composite { fields: fields.clone(), repr: rust_struct.repr, kind };

            kinds.set_kind(cs_id, TypeKind::Composite(composite));
            outcome.changed();
        }

        Ok(outcome)
    }
}
