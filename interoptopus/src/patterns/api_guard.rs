//! Defines a helper function ensuring the bindings match the used DLL.
//!
//! For details, see [`pattern_api_hash`].
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
}

unsafe impl CTypeInfo for APIVersion {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::APIVersion)
    }
}

/// Defines a helper function ensuring the bindings match the used DLL.
///
/// The generated function can be called manually and will return a `u64` hash value. In addition,
/// backends might issue an automatic version check when loading the DLL by comparing the version the
/// DLL reports with the version stored in the interop bindings.
///
/// # Hash Value
///
/// The hash value
///
/// - is based on the signatures of the involved functions, types and constants,
/// - is expected to change when the API changes, e.g., functions, types, fields, ... are added
/// changed or removed,
/// - will even react to benign API changes (e.g., just adding functions),
/// - might even react to documentation changes (subject to change; feedback welcome).
///
///
/// # Example
///
/// This will create a FFI function called `check_abi`, and backends might automatically create
/// guards calling this function when loading the DLL.
///
/// ```
/// use interoptopus::{inventory, pattern_api_guard, Library};
///
/// /// Define an inventory function `my_inventory`.
/// interoptopus::inventory!(
///     my_inventory, [],
///     // Also register `check_abi` below.
///     [ check_api ],
///     [], []
/// );
///
/// // Define a guard function called `check_api` and make it use inventory.
/// // Note: This looks circular, but isn't.
/// pattern_api_guard!(check_api, my_inventory);
///
///
/// ```
#[macro_export]
macro_rules! pattern_api_guard {
    ($name:ident, $inventory:ident) => {
        #[::interoptopus::ffi_function]
        #[no_mangle]
        pub extern "C" fn $name() -> ::interoptopus::patterns::api_guard::APIVersion {
            let library = $inventory();
            let hash = ::interoptopus::patterns::api_guard::library_hash(&library);

            ::interoptopus::patterns::api_guard::APIVersion::new(hash)
        }
    };
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
