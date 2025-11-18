//! Maps functions from Rust to C#.

use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};
use interoptopus::inventory::Functions;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "model_fn_map" } }
    }

    pub fn process(&mut self, _pass_meta: &mut super::PassMeta, x: &Functions) -> ModelResult {
        // TODO: Implement function mapping
        Ok(Unchanged)
    }
}
