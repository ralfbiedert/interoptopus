//! Provides conversion expressions for managed/unmanaged type marshalling.

use crate::lang::types::ManagedConversion;
use crate::lang::TypeId;
use crate::pass::{model, OutputResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    conversions: HashMap<TypeId, ManagedConversion>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, conversions: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        managed_conversion: &model::types::info::managed_conversion::Pass,
        kinds: &model::types::kind::Pass,
    ) -> OutputResult {
        for (type_id, _) in kinds.iter() {
            if let Some(mc) = managed_conversion.managed_conversion(*type_id) {
                self.conversions.insert(*type_id, mc);
            }
        }

        Ok(())
    }

    pub fn to_unmanaged_suffix(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => ".ToUnmanaged()",
            Some(ManagedConversion::Into) => ".IntoUnmanaged()",
            None => panic!("Unknown conversion for type {:?}", ty),
        }
    }

    pub fn to_managed_suffix(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => ".ToManaged()",
            Some(ManagedConversion::Into) => ".IntoManaged()",
            None => panic!("Unknown conversion for type {:?}", ty),
        }
    }

    pub fn to_unmanaged_name(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => "ToUnmanaged",
            Some(ManagedConversion::Into) => "IntoUnmanaged",
            None => panic!("Unknown conversion for type {:?}", ty),
        }
    }

    pub fn to_managed_name(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => "ToManaged",
            Some(ManagedConversion::Into) => "IntoManaged",
            None => panic!("Unknown conversion for type {:?}", ty),
        }
    }

    pub fn as_unmanaged_suffix(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => ".AsUnmanaged()",
            Some(ManagedConversion::Into) => ".AsUnmanaged()",
            None => panic!("Unknown conversion for type {:?}", ty),
        }
    }

}
