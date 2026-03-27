//! Holds the registered C# exception types for structured error mapping.
//!
//! The builder populates this pass with `Exception` entries. Output passes
//! (e.g. `body_from_call`) read from it to generate typed `catch` blocks.

use crate::pass::PassInfo;
use crate::pattern::Exception;

#[derive(Default)]
pub struct Config {
    pub exceptions: Vec<Exception>,
}

pub struct Pass {
    info: PassInfo,
    exceptions: Vec<Exception>,
}

impl Pass {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { info: PassInfo { name: file!() }, exceptions: config.exceptions }
    }

    #[must_use]
    pub fn exceptions(&self) -> &[Exception] {
        &self.exceptions
    }
}
