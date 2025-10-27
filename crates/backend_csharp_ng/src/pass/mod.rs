use crate::Error;

pub mod meta_info;
pub mod model_final;
pub mod model_id_maps;
pub mod model_type_map;
pub mod output_final;
pub mod output_header;
pub mod output_master;

pub type ProcessError = Result<(), Error>;
