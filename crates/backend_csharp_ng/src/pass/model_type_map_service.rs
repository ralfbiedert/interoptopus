//! Maps Rust service types to C# service types.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model_id_maps, model_type_kinds};
use interoptopus::lang;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: "model_type_map_service" },
        }
    }

    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, kinds: &mut model_type_kinds::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            match &ty.kind {
                lang::types::TypeKind::Service => {}
                _ => continue,
            }

            // Create C# TypeId for the service
            let cs_id = TypeId::from_id(rust_id.id());

            // Check if we already processed this service
            if id_map.cs_from_rust(*rust_id).is_some() {
                continue;
            }

            // Register the service type (no dependencies to check)
            id_map.set_rust_to_cs(*rust_id, cs_id);
            kinds.set_kind(cs_id, TypeKind::Service);
            outcome.changed();
        }

        Ok(outcome)
    }
}
