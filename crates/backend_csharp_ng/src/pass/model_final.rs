//! ...
//! TODO - Do we want this? If we have a "final" model, most output stages will use it,
//! and we might lose ability to reuse them in "Rust" and "Csharp" library models.

use crate::model::RustModel;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, OutputResult};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    rust_model: RustModel,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { rust_model: Default::default() }
    }

    pub fn process(&mut self) -> ModelResult {
        // TODO ...
        Ok(Unchanged)
    }
}
