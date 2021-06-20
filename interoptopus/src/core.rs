use crate::lang::c::{CType, Constant, Function};
use crate::patterns::LibraryPattern;
use crate::util::ctypes_from_functions;

/// Represents all FFI-relevant items, produced via [`inventory_function`](crate::inventory_function), ingested by backends.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Library {
    functions: Vec<Function>,
    ctypes: Vec<CType>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
}

impl Library {
    /// Produce a new library for the given functions, constants and patterns.
    ///
    /// Type information will be automatically derived from the used fields and parameters.
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

    /// Return all functions registered.
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Returns all found types; this includes types directly used in fields and parameters, and
    /// all their recursive constitutents.
    pub fn ctypes(&self) -> &[CType] {
        &self.ctypes
    }

    /// Return all registered constants.
    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    /// Return all registered [`LibraryPattern`]. In contrast, [`TypePattern`](crate::patterns::TypePattern)
    /// will be found inside the types returned via [`ctypes()`](Self::ctypes).
    pub fn patterns(&self) -> &[LibraryPattern] {
        &self.patterns
    }
}
