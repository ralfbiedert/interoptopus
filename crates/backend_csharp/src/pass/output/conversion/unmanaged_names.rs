//! Maps each type to its unmanaged name: plain name if `AsIs`, `Name.Unmanaged` otherwise.

use crate::lang::types::ManagedConversion;
use crate::model::TypeId;
use crate::pass::{model, OutputResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    names: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, names: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        names: &model::types::names::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
        kinds: &model::types::kind::Pass,
    ) -> OutputResult {
        for (type_id, _) in kinds.iter() {
            let Some(managed_name) = names.name(*type_id) else {
                continue;
            };

            let unmanaged_name = match managed_conversion.managed_conversion(*type_id) {
                Some(ManagedConversion::AsIs) => managed_name.to_string(),
                Some(_) => format!("{}.Unmanaged", managed_name),
                None => managed_name.to_string(),
            };

            self.names.insert(*type_id, unmanaged_name);
        }

        Ok(())
    }

    pub fn name(&self, ty: TypeId) -> Option<&String> {
        self.names.get(&ty)
    }
}
