//! API guards ensuring bindings match the used DLL.
//!
//! Using an API guard is as simple as defining and exporting a function `my_api_guard` returning an
//! [`Version`] as in the example below. Backends supporting API guards will automatically
//! generate guard code executed when the library is loaded.
//!
//! We **highly recommend** you add API guards, as mismatching bindings are the #1
//! cause of "inexplicable" errors (undefined behavior) that often take hours to hunt down.
//!
//! Guards are emitted automatically for plugins.
//!
//! # Example
//!
//! This will create an API guard function, and backends might automatically
//! create guards calling this function when loading the DLL.
//!
//! ```rust
//! use interoptopus::inventory::RustInventory;
//! use interoptopus::{guard, function};
//!
//! fn ffi_inventory() -> RustInventory {
//!     RustInventory::new()
//!         .register(guard!(ffi_inventory)) // <- You must name the current function.
//!         .validate()
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
use crate::inventory::RustInventory;
use crate::inventory::TypeId;
use crate::lang::meta::{Docs, Emission, FileEmission, Visibility};
use crate::lang::types::{TypeInfo, TypeKind, TypePattern, WireIO};
use crate::wire::SerializationError;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher};
use std::io::{Read, Write};

/// Holds the API version hash of the given library.
#[repr(transparent)]
#[allow(dead_code)]
#[derive(Debug, Default, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub struct Version {
    version: u64,
}

impl Version {
    /// Create a new API version from the given hash.
    #[must_use]
    pub const fn new(version: u64) -> Self {
        Self { version }
    }

    /// Mixes an additional value into this version hash, returning a new [`Version`].
    ///
    /// Use this when an important internal change (e.g., a behavioral or layout
    /// change) is not reflected in the public API signature. Bumping `salt` will
    /// force an API guard mismatch so that stale bindings are detected.
    ///
    /// ```rust
    /// use interoptopus::pattern::guard::Version;
    ///
    /// let v = Version::new(0xCAFE);
    /// let v2 = v.derive(1);
    /// assert_ne!(v, v2);
    /// ```
    #[must_use]
    pub const fn derive(self, salt: u64) -> Self {
        // FNV-1a-style mixing: XOR then multiply by a large prime.
        let mixed = (self.version ^ salt).wrapping_mul(0x100000001b3);
        Self { version: mixed }
    }

    /// Create a new API version from the given library.
    #[must_use]
    pub fn from_inventory(inventory: &RustInventory) -> Self {
        let version = Hash::from_rust(inventory).hash;
        Self { version }
    }
}

unsafe impl TypeInfo for Version {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0xA6B162106C410FCAD91327A85E3FE14E)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::Version)
    }

    fn ty() -> crate::lang::types::Type {
        crate::lang::types::Type {
            emission: Emission::FileEmission(FileEmission::Common),
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: "Version".to_string(),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl crate::inventory::Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl WireIO for Version {
    fn write(&self, w: &mut impl Write) -> Result<(), SerializationError> {
        <u64 as WireIO>::write(&self.version, w)
    }

    fn read(r: &mut impl Read) -> Result<Self, SerializationError> {
        Ok(Self { version: u64::read(r)? })
    }

    fn live_size(&self) -> usize {
        self.version.live_size()
    }
}

impl From<RustInventory> for Version {
    fn from(i: RustInventory) -> Self {
        Self::from_inventory(&i)
    }
}

/// Represents the API hash.
pub struct Hash {
    pub hash: u64,
    pub hash_hex: String,
}

impl Hash {
    /// Returns a unique hash for a Rust library inventory; used by backends.
    #[must_use]
    pub fn from_rust(inventory: &RustInventory) -> Self {
        Self::from_inventory_parts(&inventory.types, &inventory.functions, &inventory.constants)
    }

    /// Returns a unique hash for a plugin inventory; used by plugin API guards.
    #[must_use]
    pub fn from_plugin(inventory: &crate::inventory::PluginInventory) -> Self {
        Self::from_inventory_parts(&inventory.types, &inventory.functions, &inventory.constants)
    }

    fn from_inventory_parts(types: &crate::inventory::Types, functions: &crate::inventory::Functions, constants: &crate::inventory::Constants) -> Self {
        let mut hasher = DefaultHasher::new();

        let mut types: Vec<_> = types.iter().collect();
        let mut functions: Vec<_> = functions.iter().collect();
        let mut constants: Vec<_> = constants.iter().collect();

        types.sort_by_key(|(id, _)| *id);
        functions.sort_by_key(|(id, _)| *id);
        constants.sort_by_key(|(id, _)| *id);

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

/// Creates and registers an API [guard](crate::pattern::guard) for the current library.
///
/// # Example
/// ```rust
/// # use interoptopus::inventory::RustInventory;
/// # use interoptopus::{guard, function};
///
/// fn ffi_inventory() -> RustInventory {
///     RustInventory::new()
///         .register(guard!(ffi_inventory)) // <- You must name the current function.
///         .validate()                          
/// }
/// ```
#[macro_export]
macro_rules! guard {
    ($f:tt) => {{
        #[$crate::ffi]
        pub fn __api_guard() -> $crate::pattern::guard::Version {
            $f().into()
        }

        |x: &mut $crate::inventory::RustInventory| {
            <__api_guard as $crate::lang::function::FunctionInfo>::register(x);
        }
    }};
}
