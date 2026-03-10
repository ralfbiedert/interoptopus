//! Maps services from Rust to C#.

use crate::lang::service::Service;
use crate::lang::ServiceId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::try_resolve;
use interoptopus::inventory::Services;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    services: HashMap<ServiceId, Service>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, services: Default::default() }
    }

    pub fn process(&mut self, pass_meta: &mut crate::pass::PassMeta, id_map: &model::id::Pass, rs_services: &Services) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, rust_service) in rs_services {
            let Some(cs_id) = id_map.service(*rust_id) else { continue };

            if self.services.contains_key(&cs_id) {
                continue;
            }

            let cs_ty = try_resolve!(id_map.ty(rust_service.ty), pass_meta, self.info, crate::pass::MissingItem::RustType(rust_service.ty));
            let Some(cs_destructor) = id_map.fns(rust_service.destructor) else { continue };
            let mut cs_ctors = Vec::new();
            let mut all_available = true;
            let mut cs_methods = Vec::new();

            for rust_ctor in &rust_service.ctors {
                let Some(cs_fn) = id_map.fns(*rust_ctor) else {
                    all_available = false;
                    break;
                };
                cs_ctors.push(cs_fn);
            }

            if !all_available {
                continue;
            }

            for rust_method in &rust_service.methods {
                let Some(cs_fn) = id_map.fns(*rust_method) else {
                    all_available = false;
                    break;
                };
                cs_methods.push(cs_fn);
            }

            if !all_available {
                continue;
            }

            let cs_service = Service { ty: cs_ty, ctors: cs_ctors, methods: cs_methods, destructor: cs_destructor };

            self.services.insert(cs_id, cs_service);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ServiceId, &Service)> {
        self.services.iter()
    }
}
