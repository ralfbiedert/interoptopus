//! Computes C# method names for service functions (constructors, methods, destructor).
//!
//! Given a service type name like `ServiceBasic` and a function name like
//! `service_basic_do_something`, this pass strips the snake_case service prefix
//! and PascalCases the remainder to produce `DoSomething`.

use crate::lang::FunctionId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use interoptopus_backends::casing::service_method_name;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    names: HashMap<FunctionId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, names: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        service_map: &model::service::map::Pass,
        fn_map: &model::fns::all::Pass,
        types: &model::types::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (_service_id, service) in service_map.iter() {
            let Some(type_name) = types.name(service.ty) else { continue };

            let all_fns = service.ctors.iter().chain(service.methods.iter()).chain(std::iter::once(&service.destructor));

            for &fn_id in all_fns {
                if self.names.contains_key(&fn_id) {
                    continue;
                }

                let Some(func) = fn_map.get(fn_id) else { continue };

                let method_name = service_method_name(type_name, &func.name);

                self.names.insert(fn_id, method_name);
                outcome.changed();
            }
        }

        Ok(outcome)
    }

    pub fn get(&self, fn_id: FunctionId) -> Option<&str> {
        self.names.get(&fn_id).map(|s| s.as_str())
    }
}
