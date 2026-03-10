//! ...
//! TODO - Do we want this? If we have a "final" model, most output stages will use it,
//! and we might lose ability to reuse them in "Rust" and "Csharp" library models.

use crate::lang::RustPluginModel;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    rust_model: RustPluginModel,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, rust_model: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta) -> ModelResult {
        // TODO ...
        Ok(Unchanged)
    }
}
