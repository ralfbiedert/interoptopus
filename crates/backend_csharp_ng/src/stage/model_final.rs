//! ...
//! TODO - Do we want this? If we have a "final" model, most output stages will use it,
//! and we might lose ability to reuse them in "Rust" and "Csharp" library models.

use crate::model::RustModel;
use crate::stage::ProcessError;
use interoptopus::inventory::Inventory;

#[derive(Default)]
pub struct Config {}

pub struct Stage {
    rust_model: RustModel,
}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self { rust_model: Default::default() }
    }

    pub fn process(&mut self, _: &Inventory) -> ProcessError {
        // TODO ...
        Ok(())
    }
}
