//! Determines whether a type should implement `IDisposable` in C#.
//!
//! A type is disposable if its `ManagedConversion` is `Into`, meaning it
//! transfers ownership and holds native resources that must be released.

use crate::lang::TypeId;
use crate::lang::types::ManagedConversion;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    disposable: HashMap<TypeId, bool>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, disposable: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        managed_conversion: &model::types::info::managed_conversion::Pass,
        types: &model::types::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (type_id, _) in types.iter() {
            if self.disposable.contains_key(type_id) {
                continue;
            }

            let Some(mc) = managed_conversion.managed_conversion(*type_id) else {
                continue;
            };

            let is_disposable = matches!(mc, ManagedConversion::Into);
            self.disposable.insert(*type_id, is_disposable);
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn is_disposable(&self, ty: TypeId) -> Option<bool> {
        self.disposable.get(&ty).copied()
    }
}
