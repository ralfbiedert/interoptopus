use crate::model::TypeId;
use crate::Error;
use std::cmp::PartialEq;

pub mod meta_info;
pub mod model_final;
pub mod model_fn_map;
pub mod model_id_maps;
pub mod model_type_kinds;
pub mod model_type_map;
pub mod model_type_map_array;
pub mod model_type_map_delegate;
pub mod model_type_map_enum;
pub mod model_type_map_enum_variants;
pub mod model_type_map_patterns;
pub mod model_type_map_pointer;
pub mod model_type_map_primitives;
pub mod model_type_map_service;
pub mod model_type_map_struct;
pub mod model_type_map_struct_blittable;
pub mod model_type_map_struct_fields;
pub mod model_type_names;
pub mod output_final;
pub mod output_header;
pub mod output_master;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Outcome {
    Unchanged,
    Changed,
}

impl Outcome {
    pub fn changed(&mut self) {
        *self = Outcome::Changed;
    }
}

pub type ModelResult = Result<Outcome, Error>;
pub type OutputResult = Result<(), Error>;

#[derive(Debug, Copy, Clone)]
pub struct PassInfo {
    pub name: &'static str,
}

#[derive(Debug, Copy, Clone)]
pub enum MissingItem {
    CsType(TypeId),
    RustType(interoptopus::inventory::TypeId),
}

#[derive(Debug, Copy, Clone)]
pub struct Missing {
    pub origin: PassInfo,
    pub item: MissingItem,
}

#[derive(Debug, Clone, Default)]
pub struct LostAndFound {
    entries: Vec<Missing>,
}

impl LostAndFound {
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn missing(&mut self, origin: PassInfo, item: MissingItem) {
        self.entries.push(Missing { origin, item });
    }

    pub fn print(&self) {
        for missing in &self.entries {
            println!("Missing in {:?}: {:?}", missing.origin.name, missing.item);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PassMeta {
    pub lost_found: LostAndFound,
}

impl PassMeta {
    pub fn clear(&mut self) {
        self.lost_found.entries.clear();
    }
}
