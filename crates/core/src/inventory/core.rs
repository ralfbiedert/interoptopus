use crate::Error;
use crate::backend::{IndentWriter, extract_namespaces_from_types, holds_opaque_without_ref, types_from_functions_types};
use crate::lang::{Constant, Function, Type};
use crate::pattern::LibraryPattern;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;

const FORBIDDEN_NAMES: [&str; 139] = [
    "abstract",
    "add",
    "alias",
    "allows",
    "and",
    "args",
    "as",
    "ascending",
    "assert",
    "async",
    "await",
    "base",
    "bool",
    "break",
    "by",
    "byte",
    "case",
    "catch",
    "char",
    "checked",
    "class",
    "const",
    "continue",
    "decimal",
    "def",
    "default",
    "del",
    "delegate",
    "descending",
    "do",
    "double",
    "dynamic",
    "elif",
    "else",
    "enum",
    "equals",
    "event",
    "except",
    "explicit",
    "extension",
    "extern",
    "false",
    "False",
    "field",
    "file",
    "finally",
    "fixed",
    "float",
    "for",
    "foreach",
    "from",
    "get",
    "global",
    "goto",
    "group",
    "if",
    "implicit",
    "import",
    "in",
    "init",
    "int",
    "interface",
    "internal",
    "into",
    "is",
    "join",
    "lambda",
    "let",
    "lock",
    "long",
    "managed",
    "nameof",
    "namespace",
    "new",
    "nint",
    "None",
    "nonlocal",
    "not",
    "notnull",
    "nuint",
    "null",
    "object",
    "on",
    "operator",
    "or",
    "orderby",
    "out",
    "override",
    "params",
    "partial",
    "pass",
    "private",
    "protected",
    "public",
    "raise",
    "readonly",
    "record",
    "ref",
    "remove",
    "required",
    "return",
    "sbyte",
    "scoped",
    "sealed",
    "select",
    "set",
    "short",
    "signed",
    "sizeof",
    "stackalloc",
    "static",
    "string",
    "struct",
    "switch",
    "this",
    "throw",
    "true",
    "True",
    "try",
    "typedef",
    "typeof",
    "uint",
    "ulong",
    "unchecked",
    "unmanaged",
    "unsafe",
    "unsigned",
    "ushort",
    "using",
    "value",
    "var",
    "virtual",
    "void",
    "volatile",
    "when",
    "where",
    "while",
    "with",
    "yield",
];

/// Tells the [`InventoryBuilder`] what to register.
///
/// Most users won't need to touch this enum directly, as its variants are usually created via the [`function`](crate::function), [`constant`](crate::constant), [`extra_type`](crate::extra_type) and [`pattern`](crate::pattern!) macros.
#[derive(Debug)]
pub enum Symbol {
    Function(Function),
    Constant(Constant),
    Type(Type),
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
/// use interoptopus::{function, constant, extra_type, pattern, ffi_function, ffi_constant, ffi_type};
/// use interoptopus::inventory::{Inventory, InventoryBuilder};
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
///     Inventory::builder()
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
    ctypes: Vec<Type>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
    allow_reserved_names: bool,
}

impl InventoryBuilder {
    /// Start creating a new library.
    #[must_use]
    const fn new() -> Self {
        Self { functions: Vec::new(), ctypes: Vec::new(), constants: Vec::new(), patterns: Vec::new(), allow_reserved_names: false }
    }

    /// Registers a symbol.
    ///
    /// Call this with the result of a [`function`](crate::function), [`constant`](crate::constant), [`extra_type`](crate::extra_type) or [`pattern`](crate::pattern!) macro,
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
    /// generation, a panic will be raised.
    #[must_use]
    pub fn validate(self) -> Self {
        // Check for opaque parameters and return values
        for x in &self.functions {
            let has_opaque_param = x.signature().params().iter().any(|x| holds_opaque_without_ref(x.the_type()));
            assert!(!has_opaque_param, "Function `{}` has a (nested) opaque parameter. This can cause UB.", x.name());

            let has_opaque_rval = holds_opaque_without_ref(x.signature().rval());
            assert!(!has_opaque_rval, "Function `{}` has a (nested) opaque return value. This can cause UB.", x.name());
        }

        if !self.allow_reserved_names {
            validate_symbol_names(&self.functions, &self.ctypes);
        }

        self
    }

    /// Allows reserved names for inventory items.
    ///
    /// When set, you may use reserved names for your functions
    /// and fields, for example, having a field called `public`.
    ///
    /// When not set, [`self.validate`] may panic if it detects
    /// such items.
    #[must_use]
    pub fn allow_reserved_names(mut self) -> Self {
        self.allow_reserved_names = true;
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
    ctypes: Vec<Type>,
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
    CType(&'a Type),
    Constant(&'a Constant),
    Pattern(&'a LibraryPattern),
    Namespace(&'a str),
}

impl Inventory {
    /// Produce a new inventory for the given functions, constants and patterns.
    ///
    /// Type information will be automatically derived from the used fields and parameters.
    pub(crate) fn new(functions: Vec<Function>, constants: Vec<Constant>, patterns: Vec<LibraryPattern>, extra_types: &[Type]) -> Self {
        let mut ctypes = types_from_functions_types(&functions, extra_types);
        let mut namespaces = HashSet::new();

        // Extract namespace information
        extract_namespaces_from_types(&ctypes, &mut namespaces);
        namespaces.extend(functions.iter().map(|x| x.meta().module().to_string()));
        namespaces.extend(constants.iter().map(|x| x.meta().module().to_string()));

        let mut namespaces = namespaces.iter().cloned().collect::<Vec<String>>();
        namespaces.sort();

        // Dont sort functions
        // functions.sort();

        ctypes.sort();
        // constants.sort(); TODO: do sort constants (issue with Ord and float values ...)

        Self { functions, ctypes, constants, patterns, namespaces }
    }

    /// Returns a new [`InventoryBuilder`], start here.
    #[must_use]
    pub const fn builder() -> InventoryBuilder {
        InventoryBuilder::new()
    }

    /// Return all functions registered.
    #[must_use]
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Returns all found types; this includes types directly used in fields and parameters, and
    /// all their recursive constitutents.
    #[must_use]
    pub fn ctypes(&self) -> &[Type] {
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

    /// Return all registered [`LibraryPattern`]. In contrast, [`TypePattern`](crate::pattern::TypePattern)
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
    /// # use interoptopus::inventory::{Inventory, InventoryItem};
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
        let ctypes: Vec<Type> = self.ctypes.iter().filter(|x| predicate(InventoryItem::CType(x))).cloned().collect();
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

fn validate_symbol_names(functions: &[Function], ctypes: &[Type]) {
    // Check function names and parameter names.
    for func in functions {
        let name = func.name().to_lowercase();
        assert!(!FORBIDDEN_NAMES.contains(&name.as_str()), "Function `{name}` has a forbidden name that might cause issues in other languages.");

        for param in func.signature().params() {
            let param_name = param.name().to_lowercase();
            assert!(
                !FORBIDDEN_NAMES.contains(&param_name.as_str()),
                "Parameter `{param_name}` in function `{name}` has a forbidden name that might cause issues in other languages."
            );
        }
    }

    // Check type names and field/variant names.
    for ctype in ctypes {
        match ctype {
            Type::Composite(composite) => {
                let type_name = composite.rust_name();
                assert!(!FORBIDDEN_NAMES.contains(&type_name), "Type `{type_name}` has a forbidden name that might cause issues in other languages.");
                for field in composite.fields() {
                    let field_name = field.name();
                    assert!(
                        !FORBIDDEN_NAMES.contains(&field_name),
                        "Field `{field_name}` in type `{type_name}` has a forbidden name that might cause issues in other languages."
                    );
                }
            }
            Type::Enum(enum_type) => {
                let type_name = enum_type.rust_name();
                // Inline the format parameter using Rust's string interpolation.
                assert!(!FORBIDDEN_NAMES.contains(&type_name), "Enum `{type_name}` has a forbidden name that might cause issues in other languages.");
                for variant in enum_type.variants() {
                    let variant_name = variant.name();
                    assert!(
                        !FORBIDDEN_NAMES.contains(&variant_name),
                        "Variant `{variant_name}` in enum `{type_name}` has a forbidden name that might cause issues in other languages."
                    );
                }
            }
            Type::Opaque(opaque) => {
                let type_name = opaque.rust_name();
                assert!(!FORBIDDEN_NAMES.contains(&type_name), "Opaque type `{type_name}` has a forbidden name that might cause issues in other languages.");
            }
            _ => {}
        }
    }
}
