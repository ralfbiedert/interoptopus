use crate::Error;
use crate::backend::util::{ctypes_from_functions_types, extract_namespaces_from_types, holds_opaque_without_ref};
use crate::backend::writer::IndentWriter;
use crate::lang::c::{CType, Constant, Function};
use crate::patterns::LibraryPattern;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;

/// Tells the [`InventoryBuilder`] what to register.
///
/// Most users won't need to touch this enum directly, as its variants are usually created via the [`function`](crate::function), [`constant`](crate::constant), [`extra_type`](crate::extra_type) and [`pattern`](crate::pattern) macros.
#[derive(Debug)]
pub enum Symbol {
    Function(Function),
    Constant(Constant),
    Type(CType),
    Pattern(LibraryPattern),
}

/// Produces a [`Inventory`] inside your inventory function, **start here**.🔥
///
/// # Example
///
/// Define an inventory function containing a function, constant, and an extra type.
/// This function can be called from your unit tests and the returned [`Inventory`] used to create bindings.
///
/// ```rust
/// use interoptopus::{Inventory, InventoryBuilder, function, constant, extra_type, pattern, ffi_function, ffi_constant, ffi_type};
///
/// // First, define some items our DLL uses or needs.
///
/// #[ffi_function]
/// pub fn primitive_void() { }
///
/// #[ffi_constant]
/// pub const MY_CONSTANT: u32 = 123;
///
/// #[ffi_type]
/// pub struct ExtraType<T> {
///     x: T
/// }
///
/// // Then list all items for which to generate bindings. Call this function
/// // from another crate or unit test and feed the `Library` into a backend to
/// // generate bindings for a specific language.
/// pub fn my_inventory() -> Inventory {
///     InventoryBuilder::new()
///         .register(function!(primitive_void))
///         .register(constant!(MY_CONSTANT))
///         .register(extra_type!(ExtraType<f32>))
///         .validate()
///         .build()
/// }
/// ```
#[derive(Default, Debug)]
pub struct InventoryBuilder {
    functions: Vec<Function>,
    ctypes: Vec<CType>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
}

impl InventoryBuilder {
    /// Start creating a new library.
    #[must_use]
    pub const fn new() -> Self {
        Self { functions: Vec::new(), ctypes: Vec::new(), constants: Vec::new(), patterns: Vec::new() }
    }

    /// Registers a symbol.
    ///
    /// Call this with the result of a [`function`](crate::function), [`constant`](crate::constant), [`extra_type`](crate::extra_type) or [`pattern`](crate::pattern) macro,
    /// see the example above.
    #[must_use]
    pub fn register(mut self, s: Symbol) -> Self {
        match s {
            Symbol::Function(x) => self.functions.push(x),
            Symbol::Constant(x) => self.constants.push(x),
            Symbol::Type(x) => self.ctypes.push(x),
            Symbol::Pattern(x) => {
                match &x {
                    LibraryPattern::Service(x) => {
                        self.functions.push(x.destructor().clone());
                        self.functions.extend(x.constructors().iter().cloned());
                        self.functions.extend(x.methods().iter().cloned());
                    }
                    LibraryPattern::Builtins(x) => {
                        self.functions.extend(x.functions().iter().cloned());
                    }
                }
                self.patterns.push(x);
            }
        }

        self
    }

    /// Does additional sanity checking, highly recommended.
    ///
    /// This method tries to detect FFI issues that are hard to detect otherwise, and would
    /// cause issues in any backend.
    ///
    /// # Panics
    ///
    /// If a function, type, or pattern is detected that doesn't make sense in interop
    /// generation a panic will be raised.   
    #[must_use]
    pub fn validate(self) -> Self {
        for x in &self.functions {
            let has_opaque_param = x.signature().params().iter().any(|x| holds_opaque_without_ref(x.the_type()));
            assert!(!has_opaque_param, "Function `{}` has a (nested) opaque parameter. This can cause UB.", x.name());

            let has_opaque_rval = holds_opaque_without_ref(x.signature().rval());
            assert!(!has_opaque_rval, "Function `{}` has a (nested) opaque return value. This can cause UB.", x.name());
        }

        self
    }

    /// Produce the [`Inventory`].
    #[must_use]
    pub fn build(self) -> Inventory {
        Inventory::new(self.functions, self.constants, self.patterns, self.ctypes.as_slice())
    }
}

/// Holds FFI-relevant items, produced via [`InventoryBuilder`], ingested by backends.
#[derive(Clone, Debug, PartialOrd, PartialEq, Default)]
pub struct Inventory {
    functions: Vec<Function>,
    ctypes: Vec<CType>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
    namespaces: Vec<String>,
}

/// References to items contained within an [`Inventory`].
///
/// This enum is primarily used by functions such as [`Inventory::filter`] that need to handle
/// items of various types.
#[derive(Clone, Debug, PartialEq)]
pub enum InventoryItem<'a> {
    Function(&'a Function),
    CType(&'a CType),
    Constant(&'a Constant),
    Pattern(&'a LibraryPattern),
    Namespace(&'a str),
}

impl Inventory {
    /// Produce a new inventory for the given functions, constants and patterns.
    ///
    /// Type information will be automatically derived from the used fields and parameters.
    pub(crate) fn new(functions: Vec<Function>, constants: Vec<Constant>, patterns: Vec<LibraryPattern>, extra_types: &[CType]) -> Self {
        let mut ctypes = ctypes_from_functions_types(&functions, extra_types);
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

        Self { functions, ctypes, constants, patterns, namespaces }
    }

    /// Return all functions registered.
    #[must_use]
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Returns all found types; this includes types directly used in fields and parameters, and
    /// all their recursive constitutents.
    #[must_use]
    pub fn ctypes(&self) -> &[CType] {
        &self.ctypes
    }

    /// Return all registered constants.
    #[must_use]
    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    /// Return all known namespaces.
    #[must_use]
    pub fn namespaces(&self) -> &[String] {
        &self.namespaces
    }

    /// Return all registered [`LibraryPattern`]. In contrast, [`TypePattern`](crate::patterns::TypePattern)
    /// will be found inside the types returned via [`ctypes()`](Self::ctypes).
    #[must_use]
    pub fn patterns(&self) -> &[LibraryPattern] {
        &self.patterns
    }

    /// Return a new [`Inventory`] filtering items by a predicate.
    ///
    /// Useful for removing duplicate symbols when generating bindings split across multiple files.
    ///
    /// # Examples
    ///
    /// Here we filter an inventory, keeping only types, removing all other items.
    ///
    /// ```rust
    /// # use interoptopus::{Inventory, InventoryItem};
    /// #
    /// # let inventory = Inventory::default();
    /// #
    /// let filtered = inventory.filter(|x| {
    ///     match x {
    ///         InventoryItem::CType(_) => true,
    ///         _ => false,
    ///     }
    /// });
    /// ```
    #[must_use]
    pub fn filter<P: FnMut(InventoryItem) -> bool>(&self, mut predicate: P) -> Self {
        let functions: Vec<Function> = self.functions.iter().filter(|x| predicate(InventoryItem::Function(x))).cloned().collect();
        let ctypes: Vec<CType> = self.ctypes.iter().filter(|x| predicate(InventoryItem::CType(x))).cloned().collect();
        let constants: Vec<Constant> = self.constants.iter().filter(|x| predicate(InventoryItem::Constant(x))).cloned().collect();
        let patterns: Vec<LibraryPattern> = self.patterns.iter().filter(|x| predicate(InventoryItem::Pattern(x))).cloned().collect();
        let namespaces: Vec<String> = self.namespaces.iter().filter(|x| predicate(InventoryItem::Namespace(x))).cloned().collect();

        Self { functions, ctypes, constants, patterns, namespaces }
    }
}

/// Main entry point for backends to generate language bindings.
///
/// This trait will be implemented by each backend and is the main way to interface with a generator.
pub trait Bindings {
    /// Generates FFI binding code and writes them to the [`IndentWriter`].
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error>;

    /// Convenience method to write FFI bindings to the specified file with default indentation.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    fn write_file<P: AsRef<Path>>(&self, file_name: P) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = IndentWriter::new(&mut file);

        self.write_to(&mut writer)
    }

    /// Convenience method to write FFI bindings to a string.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    fn to_string(&self) -> Result<String, Error> {
        let mut vec = Vec::new();
        let mut writer = IndentWriter::new(&mut vec);
        self.write_to(&mut writer)?;
        Ok(String::from_utf8(vec)?)
    }
}
