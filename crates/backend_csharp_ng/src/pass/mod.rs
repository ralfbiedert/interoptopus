use crate::Error;
use std::cmp::PartialEq;

pub mod meta_info;
pub mod model_final;
pub mod model_id_maps;
pub mod model_type_kinds;
pub mod model_type_map;
pub mod model_type_map_array;
pub mod model_type_map_delegate;
pub mod model_type_map_enum_variants;
pub mod model_type_map_pointer;
pub mod model_type_map_primitives;
pub mod model_type_map_service;
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
