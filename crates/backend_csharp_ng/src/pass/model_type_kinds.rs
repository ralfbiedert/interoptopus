//! ...

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::ModelResult;
use crate::pass::Outcome::Unchanged;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    kinds: HashMap<TypeId, TypeKind>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { kinds: Default::default() }
    }

    pub fn process(&mut self) -> ModelResult {
        dbg!(self.kinds.len());
        Ok(Unchanged)
    }

    pub fn set_kind(&mut self, id: TypeId, kind: TypeKind) {
        self.kinds.insert(id, kind);
    }
}
