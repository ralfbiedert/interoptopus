//! ...
//! TODO - Do we want this? If we have a "final" model, most output stages will use it,
//! and we might lose ability to reuse them in "Rust" and "Csharp" library models.

use crate::model::RustModel;
use crate::pass::{ModelResult, PassInfo};
use crate::pass::Outcome::Unchanged;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    rust_model: RustModel,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: "model_final" },
            rust_model: Default::default(),
        }
    }

    pub fn process(&mut self, _pass_meta: &mut super::PassMeta) -> ModelResult {
        // TODO ...
        Ok(Unchanged)
    }
}
