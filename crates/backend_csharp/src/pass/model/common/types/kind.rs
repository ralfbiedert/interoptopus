//! ...

pub mod array;
pub mod delegate;
pub mod r#enum;
pub mod enum_variants;
pub mod opaque;
pub mod patterns;
pub mod pointer;
pub mod primitives;
pub mod service;
pub mod r#struct;
pub mod struct_fields;
pub mod wire_only;

use crate::lang::types::kind::TypeKind;
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    kinds: HashMap<TypeId, TypeKind>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, kinds: HashMap::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta) -> ModelResult {
        Ok(Unchanged)
    }

    pub fn set(&mut self, id: TypeId, kind: TypeKind) {
        self.kinds.insert(id, kind);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &TypeKind)> {
        self.kinds.iter()
    }

    #[must_use]
    pub fn contains(&self, id: &TypeId) -> bool {
        self.kinds.contains_key(id)
    }

    #[must_use]
    pub fn get(&self, id: TypeId) -> Option<&TypeKind> {
        self.kinds.get(&id)
    }
}
