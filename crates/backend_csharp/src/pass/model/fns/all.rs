//! Container for all C# functions (originals + overloads).

use crate::lang::function::Function;
use crate::lang::FunctionId;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    functions: HashMap<FunctionId, Function>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { functions: Default::default() }
    }

    pub fn register(&mut self, id: FunctionId, function: Function) {
        self.functions.insert(id, function);
    }

    pub fn get(&self, id: FunctionId) -> Option<&Function> {
        self.functions.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.functions.iter()
    }
}
