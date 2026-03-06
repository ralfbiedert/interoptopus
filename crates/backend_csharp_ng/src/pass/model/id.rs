//! Introduces C# TypeIDs and converts a Rust `TypeId` into a C# one.

use crate::model::{FunctionId, TypeId};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};
use interoptopus::inventory::Types;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    ty: HashMap<interoptopus::inventory::TypeId, TypeId>,
    fns: HashMap<interoptopus::inventory::FunctionId, FunctionId>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "model/id" }, ty: Default::default(), fns: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, _: &Types) -> ModelResult {
        Ok(Unchanged)
    }

    pub fn set_ty(&mut self, rust_id: interoptopus::inventory::TypeId, cs_id: TypeId) {
        self.ty.insert(rust_id, cs_id);
    }

    pub fn set_fns(&mut self, rust_id: interoptopus::inventory::FunctionId, cs_id: FunctionId) {
        self.fns.insert(rust_id, cs_id);
    }

    pub fn ty(&self, rust_id: interoptopus::inventory::TypeId) -> Option<TypeId> {
        self.ty.get(&rust_id).copied()
    }
}
