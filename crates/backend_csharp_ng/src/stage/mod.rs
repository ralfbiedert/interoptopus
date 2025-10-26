use crate::Error;

pub mod output_final;
pub mod output_header;
pub mod output_master;
pub mod type_id_mapping;

pub type ProcessError = Result<(), Error>;
