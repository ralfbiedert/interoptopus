//! Determines whether a C# type should be emitted as `struct` or `class`.
//!
//! Types with `ManagedConversion::AsIs` or `To` become structs; `Into` types become classes.

use crate::lang::TypeId;
use crate::lang::types::ManagedConversion;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    is_struct: HashMap<TypeId, bool>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, is_struct: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        managed_conversion: &model::types::info::managed_conversion::Pass,
        types: &model::types::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (type_id, _) in types.iter() {
            if self.is_struct.contains_key(type_id) {
                continue;
            }

            let Some(mc) = managed_conversion.managed_conversion(*type_id) else {
                continue;
            };

            self.is_struct.insert(*type_id, matches!(mc, ManagedConversion::AsIs | ManagedConversion::To));
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn is_struct(&self, ty: TypeId) -> bool {
        self.is_struct.get(&ty).copied().unwrap_or(false)
    }

    #[must_use]
    pub fn is_class(&self, ty: TypeId) -> bool {
        !self.is_struct(ty)
    }
}
