use crate::Error;
use std::cmp::PartialEq;

pub mod meta_info;
pub mod model_final;
pub mod model_id_maps;
pub mod model_type_kinds;
pub mod model_type_map;
pub mod model_type_map_primitives;
mod model_type_map_structs;
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
    pub fn cng(&self, o: &mut bool) {
        if *self == Self::Changed {
            *o = true;
        }
    }
}

pub type ModelResult = Result<Outcome, Error>;
pub type OutputResult = Result<(), Error>;
