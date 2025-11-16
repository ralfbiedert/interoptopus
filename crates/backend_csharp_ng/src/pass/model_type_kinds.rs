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
        Ok(Unchanged)
    }

    pub fn set_kind(&mut self, id: TypeId, kind: TypeKind) {
        self.kinds.insert(id, kind);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &TypeKind)> {
        self.kinds.iter()
    }

    pub fn contains(&self, id: &TypeId) -> bool {
        self.kinds.contains_key(id)
    }

    pub fn get(&self, id: TypeId) -> Option<&TypeKind> {
        self.kinds.get(&id)
    }
}
