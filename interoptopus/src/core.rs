use crate::lang::c::{CType, Constant, Function};
use crate::patterns::LibraryPattern;
use crate::util::ctypes_from_functions;

/// Represents all FFI-relevant items, produced via [`crate::inventory_function`], ingested by backends.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Library {
    functions: Vec<Function>,
    ctypes: Vec<CType>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
}

impl Library {
    pub fn new(functions: Vec<Function>, constants: Vec<Constant>, patterns: Vec<LibraryPattern>) -> Self {
        let mut ctypes = ctypes_from_functions(&functions);

        // Dont sort functions
        // functions.sort();

        ctypes.sort();
        // constants.sort(); TODO: do sort constants (issue with Ord and float values ...)

        Self {
            functions,
            ctypes,
            constants,
            patterns,
        }
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn ctypes(&self) -> &[CType] {
        &self.ctypes
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    pub fn patterns(&self) -> &[LibraryPattern] {
        &self.patterns
    }
}
