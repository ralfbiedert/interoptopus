//! Determines whether a type is nullable in C# (i.e., a reference type / class).
//!
//! Delegate classes and other reference types are nullable — when they appear as
//! struct fields, conversion code must use `?.Method() ?? default` instead of
//! `.Method()` to avoid `NullReferenceException`.

use crate::lang::TypeId;
use crate::lang::types::kind::{DelegateKind, TypeKind};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    nullable: HashMap<TypeId, bool>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, nullable: HashMap::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, types: &model::common::types::all::Pass) -> ModelResult {
        let mut outcome = Unchanged;

        for (type_id, ty) in types.iter() {
            if self.nullable.contains_key(type_id) {
                continue;
            }

            let is_nullable = matches!(&ty.kind, TypeKind::Delegate(d) if d.kind == DelegateKind::Class);
            self.nullable.insert(*type_id, is_nullable);
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn is_nullable(&self, ty: TypeId) -> Option<bool> {
        self.nullable.get(&ty).copied()
    }
}
