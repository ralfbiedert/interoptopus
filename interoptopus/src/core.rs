use crate::lang::c::{CType, Constant, Function};
use crate::patterns::LibraryPattern;
use crate::util::{ctypes_from_functions, extract_namespaces_from_types};
use std::collections::HashSet;

/// Represents all FFI-relevant items, produced via [`inventory_function`](crate::inventory_function), ingested by backends.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Library {
    functions: Vec<Function>,
    ctypes: Vec<CType>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
    namespaces: Vec<String>,
}

impl Library {
    /// Produce a new library for the given functions, constants and patterns.
    ///
    /// Type information will be automatically derived from the used fields and parameters.
    pub fn new(functions: Vec<Function>, constants: Vec<Constant>, patterns: Vec<LibraryPattern>) -> Self {
        let mut ctypes = ctypes_from_functions(&functions);
        let mut namespaces = HashSet::new();

        // Extract namespace information
        extract_namespaces_from_types(&ctypes, &mut namespaces);
        namespaces.extend(functions.iter().map(|x| x.meta().namespace().to_string()));
        namespaces.extend(constants.iter().map(|x| x.meta().namespace().to_string()));

        let mut namespaces = namespaces.iter().cloned().collect::<Vec<String>>();
        namespaces.sort();

        // Dont sort functions
        // functions.sort();

        ctypes.sort();
        // constants.sort(); TODO: do sort constants (issue with Ord and float values ...)

        Self {
            functions,
            ctypes,
            constants,
            patterns,
            namespaces,
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

    /// Return all known namespaces.
    pub fn namespaces(&self) -> &[String] {
        &self.namespaces
    }

    /// Return all registered [`LibraryPattern`]. In contrast, [`TypePattern`](crate::patterns::TypePattern)
    /// will be found inside the types returned via [`ctypes()`](Self::ctypes).
    pub fn patterns(&self) -> &[LibraryPattern] {
        &self.patterns
    }
}
