//! Provides conversion expressions for managed/unmanaged type marshalling.

use crate::lang::TypeId;
use crate::lang::types::ManagedConversion;
use crate::pass::{OutputResult, PassInfo, model};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    conversions: HashMap<TypeId, ManagedConversion>,
}

impl Pass {
    #[must_use] 
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, conversions: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        managed_conversion: &model::types::info::managed_conversion::Pass,
        types: &model::types::all::Pass,
    ) -> OutputResult {
        for (type_id, _) in types.iter() {
            if let Some(mc) = managed_conversion.managed_conversion(*type_id) {
                self.conversions.insert(*type_id, mc);
            }
        }

        Ok(())
    }

    #[must_use] 
    pub fn to_unmanaged_suffix(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => ".ToUnmanaged()",
            Some(ManagedConversion::Into) => ".IntoUnmanaged()",
            None => panic!("Unknown conversion for type {ty:?}"),
        }
    }

    #[must_use] 
    pub fn to_managed_suffix(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => ".ToManaged()",
            Some(ManagedConversion::Into) => ".IntoManaged()",
            None => panic!("Unknown conversion for type {ty:?}"),
        }
    }

    #[must_use] 
    pub fn to_unmanaged_name(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => "ToUnmanaged",
            Some(ManagedConversion::Into) => "IntoUnmanaged",
            None => panic!("Unknown conversion for type {ty:?}"),
        }
    }

    #[must_use] 
    pub fn to_managed_name(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => "ToManaged",
            Some(ManagedConversion::Into) => "IntoManaged",
            None => panic!("Unknown conversion for type {ty:?}"),
        }
    }

    #[must_use] 
    pub fn as_unmanaged_suffix(&self, ty: TypeId) -> &'static str {
        match self.conversions.get(&ty) {
            Some(ManagedConversion::AsIs) => "",
            Some(ManagedConversion::To) => ".AsUnmanaged()",
            Some(ManagedConversion::Into) => ".AsUnmanaged()",
            None => panic!("Unknown conversion for type {ty:?}"),
        }
    }
}
