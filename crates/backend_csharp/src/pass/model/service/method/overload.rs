//! Tracks which service functions (constructors and methods) have overloads.
//!
//! For each service ctor/method, this pass checks whether the underlying function
//! has any overloads registered in `fns::overload::all`. It records the function ID
//! so that output passes know a service method overload should be rendered.

use crate::lang::FunctionId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashSet;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    overloaded: HashSet<FunctionId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, overloaded: HashSet::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, services: &model::service::all::Pass, overloads: &model::fns::overload::all::Pass) -> ModelResult {
        let mut outcome = Unchanged;

        for (_service_id, service) in services.iter() {
            let all_fns = service.ctors.iter().chain(service.methods.iter());

            for &fn_id in all_fns {
                if self.overloaded.contains(&fn_id) {
                    continue;
                }

                if overloads.overloads_for(fn_id).is_some() {
                    self.overloaded.insert(fn_id);
                    outcome.changed();
                }
            }
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn has_overload(&self, fn_id: FunctionId) -> bool {
        self.overloaded.contains(&fn_id)
    }
}
