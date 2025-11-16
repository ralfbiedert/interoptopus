//! ...

use crate::lang::types::{Field, Variant};
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, model_id_maps};
use interoptopus::lang;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    fields: HashMap<TypeId, Vec<Variant>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { fields: Default::default() }
    }

    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        Ok(outcome)
    }
}
