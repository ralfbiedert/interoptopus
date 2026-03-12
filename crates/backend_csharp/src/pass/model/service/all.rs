//! Maps services from Rust to C#.

use crate::lang::ServiceId;
use crate::lang::service::Service;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
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
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, services: HashMap::default() }
    }

    pub fn process(&mut self, pass_meta: &mut crate::pass::PassMeta, id_map: &model::id_map::Pass, rs_services: &Services) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, rust_service) in rs_services {
            let Some(cs_id) = id_map.service(*rust_id) else { continue };

            if self.services.contains_key(&cs_id) {
                continue;
            }

            let cs_ty = try_resolve!(id_map.ty(rust_service.ty), pass_meta, self.info, crate::pass::MissingItem::RustType(rust_service.ty));

            let Some(cs_destructor) = id_map.fns(rust_service.destructor) else { continue };
            let Some(cs_ctors) = resolve_all(&rust_service.ctors, id_map) else { continue };
            let Some(cs_methods) = resolve_all(&rust_service.methods, id_map) else { continue };

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

fn resolve_all(rust_ids: &[interoptopus::inventory::FunctionId], id_map: &model::id_map::Pass) -> Option<Vec<crate::lang::FunctionId>> {
    rust_ids.iter().map(|&id| id_map.fns(id)).collect()
}
