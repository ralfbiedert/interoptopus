//! ...

use crate::model::Types;
use crate::pass::ProcessError;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    types: Types,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { types: Default::default() }
    }

    pub fn process(&mut self, rust_types: &interoptopus::inventory::Types) -> ProcessError {
        Ok(())
    }
}
