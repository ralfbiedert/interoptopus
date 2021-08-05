//! Helper to ensure the bindings match the used DLL.<sup>🚧</sup>
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
//!
//! # Example
//!
//! This will create a FFI function called `pattern_api_guard`, and backends might automatically
//! create guards calling this function when loading the DLL.
//!
//! ```
//! use interoptopus::ffi_function;
//! use interoptopus::patterns::api_guard::APIVersion;
//!
//! // Inventory of our exports.
//! interoptopus::inventory!(
//!     my_inventory,
//!     [],
//!     [ pattern_api_guard ],
//!     [], []
//! );
//!
//! // Guard function used by backends.
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn pattern_api_guard() -> APIVersion {
//!     APIVersion::from_library(&my_inventory())
//! }
//!
use crate::lang::c::CType;
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use crate::Library;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Holds the API version hash returned by the helper function.
#[repr(transparent)]
#[allow(dead_code)]
pub struct APIVersion {
    version: u64,
}

impl APIVersion {
    /// Create a new API version from the given hash.
    pub fn new(version: u64) -> Self {
        Self { version }
    }

    /// Create a new API version from the given library.
    pub fn from_library(library: &Library) -> Self {
        let version = library_hash(&library);
        Self { version }
    }
}

unsafe impl CTypeInfo for APIVersion {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::APIVersion)
    }
}

/// Returns a unique hash for a library.
pub fn library_hash(library: &Library) -> u64 {
    let mut hasher = DefaultHasher::new();

    // TODO: Do we need to hash patterns as well? They should never impact the 'relevant' API surface?
    let types = library.ctypes();
    let functions = library.functions();
    let constants = library.constants();

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
