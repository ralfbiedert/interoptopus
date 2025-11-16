//! ...

use crate::lang::types::{Composite, Field, TypeKind};
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, model_id_maps, model_type_kinds};
use interoptopus::lang;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    fields: HashMap<TypeId, Vec<Field>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { fields: Default::default() }
    }

    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        for (rust_id, ty) in rs_types {
            let composite = match &ty.kind {
                lang::types::TypeKind::Struct(x) => {
                    // TODO
                }
                _ => continue,
            };

            let cs_id = TypeId::from_id(rust_id.id());
            id_map.set_rust_to_cs(*rust_id, cs_id);
        }

        Ok(Unchanged)
    }
}
