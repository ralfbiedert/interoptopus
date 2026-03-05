//! ...

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::{ModelResult, PassInfo};
use crate::pass::Outcome::Unchanged;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    kinds: HashMap<TypeId, TypeKind>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: "model_type_kinds" },
            kinds: Default::default(),
        }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta) -> ModelResult {
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
