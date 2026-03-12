//! Container for all C# functions (originals + overloads).

use crate::lang::FunctionId;
use crate::lang::functions::Function;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

#[derive(Debug)]
pub struct Pass {
    functions: HashMap<FunctionId, Function>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { functions: HashMap::default() }
    }

    pub fn register(&mut self, id: FunctionId, function: Function) {
        self.functions.insert(id, function);
    }

    #[must_use]
    pub fn get(&self, id: FunctionId) -> Option<&Function> {
        self.functions.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.functions.iter()
    }
}
