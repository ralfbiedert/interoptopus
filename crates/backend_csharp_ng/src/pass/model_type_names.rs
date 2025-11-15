//! ...

use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, model_id_maps};

#[derive(Default)]
pub struct Config {}

pub struct Pass {}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self) -> ModelResult {
        Ok(Unchanged)
    }
}
