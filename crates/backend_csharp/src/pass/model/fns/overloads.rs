//! Produces overload variants for C# functions.
//!
//! Currently a placeholder. Will later query originals, produce overloads,
//! and register them with the `all` pass.

#[derive(Default)]
pub struct Config {}

pub struct Pass {
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {}
    }
}
