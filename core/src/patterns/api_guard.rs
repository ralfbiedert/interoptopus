//! Helper to ensure the bindings match the used DLL.
//!
//! Using an API guard is as simple as defining and exporting a function `my_api_guard` returning an
//! [`APIVersion`] as in the example below. Backends supporting API guards will automatically
//! generate guard code executed when the library is loaded.
//!
//! When developing we **highly recommend** adding API guards, as mismatching bindings are the #1
//! cause of "inexplicable" errors (undefined behavior) that often take hours to hunt down.
//!
//! # Example
//!
//! This will create a FFI function called `pattern_api_guard`, and backends might automatically
//! create guards calling this function when loading the DLL.
//!
//! ```
//! use interoptopus::{ffi_function, Inventory, InventoryBuilder, function};
//! use interoptopus::patterns::api_guard::APIVersion;
//!
//! // Guard function used by backends.
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn my_api_guard() -> APIVersion {
//!     my_inventory().into()
//! }
//!
//! // Inventory of our exports.
//! pub fn my_inventory() -> Inventory {
//!     InventoryBuilder::new()
//!         .register(function!(my_api_guard))
//!         .inventory()
//! }
//! ```
//!
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
//! changed or removed,
//! - will even react to benign API changes (e.g., just adding functions),
//! - might even react to documentation changes (subject to change; feedback welcome).
//!
use crate::lang::c::CType;
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use crate::Inventory;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Holds the API version hash of the given library.
#[repr(transparent)]
#[allow(dead_code)]
#[derive(Debug, Default, PartialOrd, PartialEq, Copy, Clone)]
pub struct APIVersion {
    version: u64,
}

impl APIVersion {
    /// Create a new API version from the given hash.
    pub fn new(version: u64) -> Self {
        Self { version }
    }

    /// Create a new API version from the given library.
    pub fn from_inventory(inventory: &Inventory) -> Self {
        let version = inventory_hash(inventory);
        Self { version }
    }
}

unsafe impl CTypeInfo for APIVersion {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::APIVersion)
    }
}

impl From<Inventory> for APIVersion {
    fn from(i: Inventory) -> Self {
        Self::from_inventory(&i)
    }
}
/// Returns a unique hash for an inventory; used by backends.
pub fn inventory_hash(inventory: &Inventory) -> u64 {
    let mut hasher = DefaultHasher::new();

    // TODO: Do we need to hash patterns as well? They should never impact the 'relevant' API surface?
    let types = inventory.ctypes();
    let functions = inventory.functions();
    let constants = inventory.constants();

    // TODO: Should probably exclude documentation & co.
    for t in types {
        t.hash(&mut hasher);
    }

    // TODO: Should probably exclude documentation & co.
    for f in functions {
        f.hash(&mut hasher);
    }

    for c in constants {
        c.name().hash(&mut hasher);
        c.value().fucking_hash_it_already(&mut hasher);
    }

    hasher.finish()
}
