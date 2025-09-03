use crate::inventory::forbidden::FORBIDDEN_NAMES;
use crate::lang::util::{extract_namespaces_from_types, extract_wire_types_from_functions, holds_opaque_without_ref, types_from_functions_types};
use crate::lang::{Constant, Function, Type};
use crate::pattern::LibraryPattern;
use std::collections::HashSet;

/// Tells the [`InventoryBuilder`] what to register.
///
/// Most users won't need to touch this enum directly, as its variants are usually created via the [`function`](crate::function), [`constant`](crate::constant), [`extra_type`](crate::extra_type) and [`pattern`](crate::pattern!) macros.
#[derive(Debug)]
pub enum Symbol {
    Function(Function),
    Constant(Constant),
    Type(Type),
    Pattern(LibraryPattern),
    /// An externally defined type.  Will not be defined within this file.
    /// Used when you want to insert includes rather than having local definitions.
    ExternType(String),
}

/// Produces an [`Inventory`] inside your inventory function, **start here**.🔥
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
    extra_types: Vec<Type>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
    allow_reserved_names: bool,
    extern_types: Vec<String>,
}

impl InventoryBuilder {
    /// Start creating a new library.
    #[must_use]
    const fn new() -> Self {
        Self {
            functions: Vec::new(),
            extra_types: Vec::new(),
            /*wire_types: Vec::new(),*/ constants: Vec::new(),
            patterns: Vec::new(),
            allow_reserved_names: false,
            extern_types: Vec::new(),
        }
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
            Symbol::Type(x) => self.extra_types.push(x),
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
            Symbol::ExternType(et) => self.extern_types.push(et),
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
            validate_symbol_names(&self.functions, &self.extra_types);
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
        Inventory::new(self.functions, self.constants, self.patterns, self.extra_types.as_slice(), self.extern_types)
    }
}

/// Holds FFI-relevant items, produced via [`InventoryBuilder`], ingested by backends.
#[derive(Clone, Debug, PartialOrd, PartialEq, Default)]
pub struct Inventory {
    functions: Vec<Function>,
    /// FFI types and Domain types.
    c_types: Vec<Type>,
    /// These are the types explicitly marked as `Wire<T>` in function declarations, we extract them here so we can
    /// rebuild the chain of custody and generate all appropriate types.
    /// Other wired types contained within these listed ones are already added to `c_types` as `Domain` types.
    wire_types: Vec<Type>,
    constants: Vec<Constant>,
    patterns: Vec<LibraryPattern>,
    namespaces: Vec<String>,
    /// Types that are not to be defined within the output, but rather included via headers.
    extern_types: Vec<String>,
}

/// References to items contained within an [`Inventory`].
///
/// This enum is primarily used by functions such as [`Inventory::filter`] that need to handle
/// items of various types.
#[derive(Clone, Debug, PartialEq)]
pub enum InventoryItem<'a> {
    Function(&'a Function),
    CType(&'a Type),
    WireType(&'a Type),
    Constant(&'a Constant),
    Pattern(&'a LibraryPattern),
    Namespace(&'a str),
    ExternType(String),
}

fn dedup<T: std::hash::Hash + Eq>(v: Vec<T>) -> Vec<T> {
    v.into_iter().collect::<HashSet<T>>().into_iter().collect::<Vec<T>>()
}

// fn dedup_sort<T: Ord>(mut v: Vec<T>) -> Vec<T> {
//     v.sort();
//     v.dedup();
//     v
// }

impl Inventory {
    /// Produce a new inventory for the given functions, constants and patterns.
    ///
    /// Type information will be automatically derived from the used fields and parameters.
    pub(crate) fn new(functions: Vec<Function>, constants: Vec<Constant>, patterns: Vec<LibraryPattern>, extra_types: &[Type], mut extern_types: Vec<String>) -> Self {
        let mut c_types = types_from_functions_types(&functions, extra_types);
        let mut namespaces = HashSet::new();

        // Extract namespace information
        extract_namespaces_from_types(&c_types, &mut namespaces);
        namespaces.extend(functions.iter().map(|x| x.meta().module().to_string()));
        namespaces.extend(constants.iter().map(|x| x.meta().module().to_string()));

        let mut namespaces = namespaces.iter().cloned().collect::<Vec<String>>();
        namespaces.sort();

        let wire_types = dedup(extract_wire_types_from_functions(&functions));

        // Dont sort functions
        // functions.sort();

        c_types.sort();
        // constants.sort(); TODO: do sort constants (issue with Ord and float values ...)

        extern_types.sort();

        Self { functions, c_types, wire_types, constants, patterns, namespaces, extern_types }
    }

    /// Helper to debug the inventory.
    pub fn debug(&self) {
        eprintln!("✅ Inventory check");
        for ns in &self.namespaces {
            eprintln!("💳 {ns}");
        }
        for cs in &self.constants {
            eprintln!("👮🏼‍♀️ {}", cs.name());
        }
        for f in &self.functions {
            eprintln!(
                "🧵 {}.{}({}) -> {}",
                f.meta().module(),
                f.name(),
                f.signature().params().iter().map(|p| p.the_type().name_within_lib()).collect::<Vec::<_>>().join(","),
                f.signature().rval().name_within_lib()
            );
        }
        for c in &self.c_types {
            eprintln!("🎉 {}", c.name_within_lib());
        }
        for w in &self.wire_types {
            eprintln!("🔧 {} '{}'", w.name_within_lib(), w.namespace().unwrap_or_default());
        }
        // for p in &self.patterns {
        //     eprintln!("🧵 {}", p.fallback_type().name_within_lib());
        // }
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
    /// all their recursive constituents.
    #[must_use]
    pub fn c_types(&self) -> &[Type] {
        &self.c_types
    }

    /// Returns initial wire types; this includes wire types directly used in function parameters.
    /// To find all transitively wired types, look for `wire_domain_types()`
    #[must_use]
    pub fn wire_types(&self) -> &[Type] {
        &self.wire_types
    }

    /// Returns domain wire types; this includes types `T` directly and indirectly used in `Wire<T>` function parameters.
    #[must_use]
    #[allow(clippy::redundant_closure_for_method_calls)]
    pub fn wire_domain_types(&self) -> Vec<Type> {
        let trans_types = self
            .wire_types
            .iter()
            .flat_map(|wt| match wt {
                Type::Wire(w) => w
                    .fields()
                    .iter()
                    .filter(|&f| matches!(f.the_type(), Type::Composite(_)))
                    .map(|f| f.the_type())
                    .filter(|ty| self.c_types.contains(ty)),
                _ => panic!("What's a non-wired type doing here?"),
            })
            .cloned()
            .collect::<HashSet<Type>>();
        trans_types.into_iter().collect()
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
    /// will be found inside the types returned via [`c_types()`](Self::c_types).
    #[must_use]
    pub fn patterns(&self) -> &[LibraryPattern] {
        &self.patterns
    }

    /// Return all registered extern types.
    #[must_use]
    pub fn extern_types(&self) -> &[String] {
        &self.extern_types
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
        let c_types: Vec<Type> = self.c_types.iter().filter(|x| predicate(InventoryItem::CType(x))).cloned().collect();
        let wire_types: Vec<Type> = self.wire_types.iter().filter(|x| predicate(InventoryItem::WireType(x))).cloned().collect();
        let constants: Vec<Constant> = self.constants.iter().filter(|x| predicate(InventoryItem::Constant(x))).cloned().collect();
        let patterns: Vec<LibraryPattern> = self.patterns.iter().filter(|x| predicate(InventoryItem::Pattern(x))).cloned().collect();
        let namespaces: Vec<String> = self.namespaces.iter().filter(|x| predicate(InventoryItem::Namespace(x))).cloned().collect();
        let extern_types: Vec<String> = self
            .extern_types
            .iter()
            .filter(|x| predicate(InventoryItem::ExternType((*x).clone())))
            .cloned()
            .collect();

        Self { functions, c_types, wire_types, constants, patterns, namespaces, extern_types }
    }

    /// Return a new [`Inventory`] after filtering and modifying items.
    ///
    /// Used mostly in order to deal with items you wish to be undefined within the output.
    /// A potential use case is to have a single file for an error type enum shared by multiple
    /// generated headers which will include the error type rather than redefining it in each header.
    /// As such, you would remap the enum definition to a `Extern` type.
    ///
    /// # Examples
    ///
    /// Here we will look for `error_t` enum types and remap them to `Extern` types to prevent
    /// them from being defined in the output.  Note: the functions still have the entire enum
    /// declaration embedded within the signatures.  But the Extern type will prevent them from
    /// propagating into a definition.
    ///
    /// ```rust
    /// # use interoptopus::inventory::{Inventory, InventoryItem};
    /// #
    /// # let inventory = Inventory::default();
    /// #
    /// let replaced = inventory.filter_map(|item| {
    ///     match item {
    ///       InventoryItem::CType(t) if t.name_within_lib() == "error_t" => {
    ///         Some(InventoryItem::ExternType(t.name_within_lib()))
    ///       }
    ///       _ => Some(item)
    ///     }
    /// });
    /// ```
    #[must_use]
    pub fn filter_map<P: FnMut(InventoryItem<'_>) -> Option<InventoryItem<'_>>>(self, mut predicate: P) -> Self {
        // Iterate over all items collecting them into a new vector if the predicate returns Some.
        // Remap the items back into an Inventory since they may have changed type.
        let mut result = Self::default();

        self.functions.into_iter().for_each(|f| result.insert(predicate(InventoryItem::Function(&f))));
        self.c_types.into_iter().for_each(|t| result.insert(predicate(InventoryItem::CType(&t))));
        self.wire_types.into_iter().for_each(|t| result.insert(predicate(InventoryItem::WireType(&t))));
        self.constants.into_iter().for_each(|c| result.insert(predicate(InventoryItem::Constant(&c))));
        self.patterns.into_iter().for_each(|p| result.insert(predicate(InventoryItem::Pattern(&p))));
        self.namespaces.iter().for_each(|n| result.insert(predicate(InventoryItem::Namespace(n))));
        self.extern_types
            .iter()
            .for_each(|et| result.insert(predicate(InventoryItem::ExternType(et.clone()))));

        result
    }

    /// Internal helper to remap an InventoryItem back into an owned item.
    fn insert(&mut self, item: Option<InventoryItem<'_>>) {
        if let Some(item) = item {
            match item {
                InventoryItem::Function(f) => self.functions.push(f.clone()),
                InventoryItem::CType(t) => self.c_types.push(t.clone()),
                InventoryItem::WireType(t) => self.wire_types.push(t.clone()),
                InventoryItem::Constant(c) => self.constants.push(c.clone()),
                InventoryItem::Pattern(p) => self.patterns.push(p.clone()),
                InventoryItem::Namespace(n) => self.namespaces.push(n.to_string()),
                InventoryItem::ExternType(et) => self.extern_types.push(et.to_string()),
            }
        }
    }
}

fn validate_symbol_names(functions: &[Function], c_types: &[Type]) {
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
    for ctype in c_types {
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
