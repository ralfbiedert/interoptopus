//! Extends service structures with overloaded function IDs for constructors and methods.
//!
//! For each service ctor/method, this pass checks whether the function has a simple
//! overload or a body overload. If it does, the overloaded FunctionId is stored so that
//! output passes can pick the best variant when rendering service class methods.
//!
//! Priority: body overload (delegate) > simple overload (ref) > original (no entry).

use crate::lang::FunctionId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

/// Which overload variant applies to a service function.
#[derive(Clone, Debug)]
pub enum OverloadKind {
    /// A simple overload exists (e.g., IntPtr replaced by ref).
    Simple(FunctionId),
    /// A body overload exists (e.g., delegate wrapping).
    Body,
}

pub struct Pass {
    info: PassInfo,
    overloads: HashMap<FunctionId, OverloadKind>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, overloads: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        service_map: &model::service::map::Pass,
        overload_simple: &model::fns::overload::simple::Pass,
        overload_body: &model::fns::overload::body::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (_service_id, service) in service_map.iter() {
            let all_fns = service.ctors.iter().chain(service.methods.iter());

            for &fn_id in all_fns {
                if self.overloads.contains_key(&fn_id) {
                    continue;
                }

                // Body overload takes priority over simple overload
                if overload_body.transforms(fn_id).is_some() {
                    self.overloads.insert(fn_id, OverloadKind::Body);
                    outcome.changed();
                } else if let Some(ids) = overload_simple.overloads_for(fn_id) {
                    if let Some(&simple_id) = ids.first() {
                        self.overloads.insert(fn_id, OverloadKind::Simple(simple_id));
                        outcome.changed();
                    }
                }
            }
        }

        Ok(outcome)
    }

    pub fn get(&self, fn_id: FunctionId) -> Option<&OverloadKind> {
        self.overloads.get(&fn_id)
    }
}
