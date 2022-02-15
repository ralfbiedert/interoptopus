use crate::lang::c::{CType, Constant, Function};
use crate::patterns::LibraryPattern;
use crate::util::{ctypes_from_functions_types, extract_namespaces_from_types};
use std::collections::HashSet;

/// Tells the [`LibraryBuilder`] what to register.
///
/// Most users won't need to touch this enum directly, as its variants are usually created via the [`function`](crate::function), [`constant`](crate::constant), [`extra_type`](crate::ctype) and [`pattern`](crate::pattern) macros.
#[derive(Debug)]
pub enum Symbol {
    Function(Function),
    Constant(Constant),
    Type(CType),
    Pattern(LibraryPattern),
}

/// Produces a [`Library`] inside your inventory function, **start here**.
///
///
///
/// # Example
///
/// Define an inventory function containing a function, constant, and an extra type.
/// This function can be called from your unit tests and the returned [`Library`] used to create bindings.
///
/// ```rust
/// use interoptopus::{Library, LibraryBuilder, function, constant, extra_type, pattern, ffi_function, ffi_constant, ffi_type};
///
/// // First, define some items our DLL uses or needs.
///
/// #[ffi_function]
/// #[no_mangle]
/// pub extern "C" fn primitive_void() { }
///
/// #[ffi_constant]
/// pub const MY_CONSTANT: u32 = 123;
///
/// #[ffi_type]
/// #[repr(C)]
/// pub struct ExtraType<T> {
///     x: T
/// }
///
/// // Then list all items for which to generate bindings. Call this function
/// // from another crate or unit test and feed the `Library` into a backend to
/// // generate bindings for a specific language.
/// pub fn my_inventory() -> Library {
///     LibraryBuilder::new()
///         .register(function!(primitive_void))
///         .register(constant!(MY_CONSTANT))
///         .register(extra_type!(ExtraType<f32>))
///         .library()
/// }
/// ```
#[derive(Default, Debug)]
pub struct LibraryBuilder {
    functions: Vec<Function>,
    ctypes: Vec<CType>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
}

impl LibraryBuilder {
    /// Start creating a new library.
    pub fn new() -> Self {
        LibraryBuilder {
            functions: Vec::new(),
            ctypes: Vec::new(),
            constants: Vec::new(),
            patterns: Vec::new(),
        }
    }

    /// Registers a symbol.
    ///
    /// Call this with the result of a [`function`](crate::function), [`constant`](crate::constant), [`extra_type`](crate::extra_type) or [`pattern`](crate::pattern) macro,
    /// see the example above.
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
                }
                self.patterns.push(x)
            }
        }

        self
    }

    /// Produce the [`Library`].
    pub fn library(self) -> Library {
        Library::new(self.functions, self.constants, self.patterns, self.ctypes)
    }
}

/// Represents all FFI-relevant items, produced via [`LibraryBuilder`], ingested by backends.
#[derive(Clone, Debug, PartialOrd, PartialEq, Default)]
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
    fn new(functions: Vec<Function>, constants: Vec<Constant>, patterns: Vec<LibraryPattern>, extra_types: Vec<CType>) -> Self {
        let mut ctypes = ctypes_from_functions_types(&functions, &extra_types);
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

/// Returns all functions not belonging to a [`service`](crate::patterns::service) pattern.
///
/// Useful in backends like Python that can fully encapsulate services and should not expose their
/// raw methods in the main namespace.
pub fn non_service_functions(library: &Library) -> Vec<&Function> {
    let mut service_methods = vec![];
    for pattern in library.patterns() {
        match pattern {
            LibraryPattern::Service(service) => {
                service_methods.extend_from_slice(service.methods());
                service_methods.extend_from_slice(service.constructors());
                service_methods.push(service.destructor().clone());
            }
        }
    }

    library.functions().iter().filter(|&x| !service_methods.contains(x)).collect()
}

/// Create a single [`Library`](Library) from a number of individual libraries.
///
/// This function can be useful when your FFI crate exports different sets of
/// symbols (e.g., _core_ and _extension_ functions) and you want to create different
/// bindings based on some compile target or configuration
///
/// # Example
///
/// ```
/// # mod my_crate {
/// #     use interoptopus::Library;
/// #     pub fn inventory_core() -> Library { Library::default() }
/// #     pub fn inventory_ext() -> Library { Library::default() }
/// # }
/// use interoptopus::merge_libraries;
///
/// let libraries = [
///     my_crate::inventory_core(),
///     my_crate::inventory_ext()
/// ];
///
/// merge_libraries(&libraries);
/// ```
pub fn merge_libraries(libraries: &[Library]) -> Library {
    let mut functions = Vec::new();
    let mut constants = Vec::new();
    let mut patterns = Vec::new();
    let mut types = Vec::new();

    for library in libraries {
        functions.extend_from_slice(library.functions());
        constants.extend_from_slice(library.constants());
        patterns.extend_from_slice(library.patterns());
        types.extend_from_slice(library.ctypes());
    }

    Library::new(functions, constants, patterns, types)
}
