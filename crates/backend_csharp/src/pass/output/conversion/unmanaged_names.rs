//! Maps each type to its unmanaged name: plain name if `AsIs`, `Name.Unmanaged` otherwise.

use crate::lang::TypeId;
use crate::lang::types::ManagedConversion;
use crate::pass::{OutputResult, PassInfo, model};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    names: HashMap<TypeId, String>,
}

impl Pass {
    #[must_use] 
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, names: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        types: &model::types::all::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
    ) -> OutputResult {
        for (type_id, ty) in types.iter() {
            let managed_name = &ty.name;

            let unmanaged_name = match managed_conversion.managed_conversion(*type_id) {
                Some(ManagedConversion::AsIs) => managed_name.clone(),
                Some(_) => format!("{managed_name}.Unmanaged"),
                None => managed_name.clone(),
            };

            self.names.insert(*type_id, unmanaged_name);
        }

        Ok(())
    }

    #[must_use] 
    pub fn name(&self, ty: TypeId) -> Option<&String> {
        self.names.get(&ty)
    }
}
