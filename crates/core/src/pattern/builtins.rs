#![doc(hidden)]

use crate::lang::Function;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Builtins {
    functions: Vec<Function>,
}

impl Builtins {
    #[must_use]
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }

    #[must_use]
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }
}
