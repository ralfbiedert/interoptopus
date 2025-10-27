//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::model::TypeId;
use crate::pass::ProcessError;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    rust_to_cs: HashMap<interoptopus::inventory::TypeId, TypeId>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { rust_to_cs: Default::default() }
    }

    pub fn process(&mut self, rs_types: &interoptopus::inventory::Types) -> ProcessError {
        // TODO: Are we ok randomly assigning a new C# type ID to "well known" C# types?
        //       For example, should `String` get a new ID each run, or always the same?
        for (id, ty) in rs_types {
            let cs_id = TypeId::from_id(id.id());
            self.rust_to_cs.insert(*id, cs_id);
        }
        Ok(())
    }
}
