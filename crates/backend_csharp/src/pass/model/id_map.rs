//! Introduces C# `TypeIDs` and converts a Rust `TypeId` into a C# one.
//!
//! Populates all type and function id mappings upfront. The C# `TypeId` is
//! always `TypeId::from_id(rust_id.id())`, except for two special pattern
//! types (`CStrPointer`, `Utf8String`) which have predefined C# `TypeIds`.

use crate::lang::types::csharp;
use crate::lang::{FunctionId, ServiceId, TypeId};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};
use interoptopus::inventory::{Functions, Services, Types};
use interoptopus::lang;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

#[derive(Debug)]
pub struct Pass {
    info: PassInfo,
    ty: HashMap<interoptopus::inventory::TypeId, TypeId>,
    fns: HashMap<interoptopus::inventory::FunctionId, FunctionId>,
    services: HashMap<interoptopus::inventory::ServiceId, ServiceId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, ty: HashMap::default(), fns: HashMap::default(), services: HashMap::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, rs_types: &Types, rs_functions: &Functions, rs_services: &Services) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            if self.ty.contains_key(rust_id) {
                continue;
            }

            let cs_id = match &ty.kind {
                lang::types::TypeKind::TypePattern(lang::types::TypePattern::CStrPointer) => csharp::CSTR_PTR,
                lang::types::TypeKind::TypePattern(lang::types::TypePattern::Utf8String) => csharp::UTF8_STRING,
                _ => TypeId::from_id(rust_id.id()),
            };

            self.ty.insert(*rust_id, cs_id);
            outcome.changed();
        }

        for rust_id in rs_functions.keys() {
            if self.fns.contains_key(rust_id) {
                continue;
            }

            let cs_id = FunctionId::from_id(rust_id.id());
            self.fns.insert(*rust_id, cs_id);
            outcome.changed();
        }

        for rust_id in rs_services.keys() {
            if self.services.contains_key(rust_id) {
                continue;
            }

            let cs_id = ServiceId::from_id(rust_id.id());
            self.services.insert(*rust_id, cs_id);
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn ty(&self, rust_id: interoptopus::inventory::TypeId) -> Option<TypeId> {
        self.ty.get(&rust_id).copied()
    }

    #[must_use]
    pub fn fns(&self, rust_id: interoptopus::inventory::FunctionId) -> Option<FunctionId> {
        self.fns.get(&rust_id).copied()
    }

    #[must_use]
    pub fn service(&self, rust_id: interoptopus::inventory::ServiceId) -> Option<ServiceId> {
        self.services.get(&rust_id).copied()
    }
}
