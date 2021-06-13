use crate::lang::c::{CType, Constant, Function};
use crate::util::types_from_functions;


/// Represents all FFI-relevant items, produced via [`crate::inventory_function`], ingested by backends.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Library {
    functions: Vec<Function>,
    types: Vec<CType>,
    constants: Vec<Constant>,
}

impl Library {
    pub fn new(mut functions: Vec<Function>, constants: Vec<Constant>) -> Self {
        let mut types = types_from_functions(&functions);

        functions.sort();
        types.sort();
        // constants.sort(); TODO

        Self { functions, types, constants }
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn types(&self) -> &[CType] {
        &self.types
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
}
