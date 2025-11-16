//! Maps Rust pointers (ReadPointer, ReadWritePointer) to C# pointers.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
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
            let rust_pointee_id = match &ty.kind {
                lang::types::TypeKind::ReadPointer(pointee) => pointee,
                lang::types::TypeKind::ReadWritePointer(pointee) => pointee,
                _ => continue,
            };

            // Create C# TypeId for the pointer
            let cs_id = TypeId::from_id(rust_id.id());

            // Check if we already processed this pointer
            if id_map.cs_from_rust(*rust_id).is_some() {
                continue;
            }

            // Try to convert the pointee type
            let Some(cs_pointee_id) = id_map.cs_from_rust(*rust_pointee_id) else {
                continue;
            };

            // Register the pointer type
            id_map.set_rust_to_cs(*rust_id, cs_id);
            kinds.set_kind(cs_id, TypeKind::Pointer(cs_pointee_id));
            outcome.changed();
        }

        Ok(outcome)
    }
}
