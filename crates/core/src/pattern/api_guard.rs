//! Ensures bindings match the used DLL, used via [`api_guard!`](crate::api_guard).
//!
//! Using an API guard is as simple as defining and exporting a function `my_api_guard` returning an
//! [`ApiVersion`] as in the example below. Backends supporting API guards will automatically
//! generate guard code executed when the library is loaded.
//!
//! When developing we **highly recommend** adding API guards, as mismatching bindings are the #1
//! cause of "inexplicable" errors (undefined behavior) that often take hours to hunt down.
//!
//! # Example
//!
//! This will create an API guard function, and backends might automatically
//! create guards calling this function when loading the DLL.
//!
//! ```rust
//! use interoptopus::inventory::Inventory;
//! use interoptopus::{api_guard, ffi_function, function};
//!
//! fn ffi_inventory() -> Inventory {
//!     Inventory::builder()
//!         .register(api_guard!(ffi_inventory)) // <- You must name the current function.
//!         .validate()                          //    since it will be called at runtime
//!         .build()                             //    but cannot be inferred.
//! }
//! ```
//! In backends that support API guards an error message like this might be emitted if you try load
//! a library with mismatching bindings:
//!
//! ```csharp
//! Exception: API reports hash X which differs from hash in bindings (Y). You probably forgot to update / copy either the bindings or the library.
//! ```
//!
//!
//! # Hash Value
//!
//! The hash value
//!
//! - is based on the signatures of the involved functions, types and constants,
//! - is expected to change when the API changes, e.g., functions, types, fields, ... are added
//!   changed or removed,
//! - will even react to benign API changes (e.g., just adding functions),
//! - might even react to documentation changes (subject to change; feedback welcome).
//!
use crate::inventory::Inventory;
use crate::inventory::TypeId;
use crate::lang::meta::{Docs, Emission, Visibility};
use crate::lang::types::{TypeKind, TypePattern};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Holds the API version hash of the given library.
#[repr(transparent)]
#[allow(dead_code)]
#[derive(Debug, Default, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub struct ApiVersion {
    version: u64,
}

impl ApiVersion {
    /// Create a new API version from the given hash.
    #[must_use]
    pub const fn new(version: u64) -> Self {
        Self { version }
    }

    /// Create a new API version from the given library.
    #[must_use]
    pub fn from_inventory(inventory: &Inventory) -> Self {
        let version = ApiHash::from(inventory).hash;
        Self { version }
    }
}

impl crate::lang::types::TypeInfo for ApiVersion {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;

    fn id() -> TypeId {
        TypeId::new(0xA6B162106C410FCAD91327A85E3FE14E)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::APIVersion)
    }

    fn ty() -> crate::lang::types::Type {
        crate::lang::types::Type {
            emission: Emission::Common,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: "ApiVersion".to_string(),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut crate::inventory::Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl crate::lang::Register for ApiVersion {
    fn register(inventory: &mut crate::inventory::Inventory) {
        <Self as crate::lang::types::TypeInfo>::register(inventory);
    }
}

impl From<Inventory> for ApiVersion {
    fn from(i: Inventory) -> Self {
        Self::from_inventory(&i)
    }
}

/// Represents the API hash.
pub struct ApiHash {
    pub hash: u64,
    pub hash_hex: String,
}

impl ApiHash {
    /// Returns a unique hash for an inventory; used by backends.
    #[must_use]
    pub fn from(inventory: &Inventory) -> Self {
        let mut hasher = DefaultHasher::new();

        let types = inventory.types.iter();
        let functions = inventory.functions.iter();
        let constants = inventory.constants.iter();

        for t in types {
            t.hash(&mut hasher);
        }

        for f in functions {
            f.hash(&mut hasher);
        }

        for c in constants {
            c.1.name.hash(&mut hasher);
            c.1.value.hash(&mut hasher);
        }

        Self::new(hasher.finish())
    }

    /// Creates a new hash from the given raw hash value.
    #[must_use]
    pub fn new(hash: u64) -> Self {
        let hash_hex = format!("{hash:x}");
        Self { hash, hash_hex }
    }

    #[must_use]
    pub const fn hash(&self) -> u64 {
        self.hash
    }

    #[must_use]
    pub fn hash_hex(&self) -> &str {
        self.hash_hex.as_str()
    }
}

/// Creates and registers an [API guard](crate::pattern::api_guard) for the current library.
///
/// # Example
/// ```rust
/// # use interoptopus::inventory::Inventory;
/// # use interoptopus::{api_guard, ffi_function, function};
///
/// fn ffi_inventory() -> Inventory {
///     Inventory::builder()
///         .register(api_guard!(ffi_inventory)) // <- You must name the current function.
///         .validate()                          //    since it will be called at runtime
///         .build()                             //    but cannot be inferred.
/// }
/// ```
#[macro_export]
macro_rules! api_guard {
    ($f:tt) => {{
        #[$crate::ffi_function]
        pub fn __api_guard() -> $crate::pattern::api_guard::ApiVersion {
            $f().into()
        }

        use $crate::lang::FunctionInfo;
        let info = __api_guard::function_info();
        $crate::inventory::Symbol::Function(info)
    }};
}

#[macro_export]
macro_rules! api_guard2 {
    ($f:tt) => {{
        #[$crate::ffi_function]
        pub fn __api_guard() -> $crate::pattern::api_guard::ApiVersion {
            $f().into()
        }

        use $crate::lang::Register;

        |x: &mut $crate::inventory::Inventory| {
            // TODO
            // - register __api_guard (this should automatically register the ApiGuard type)
            todo ...
        }
    }};
}
