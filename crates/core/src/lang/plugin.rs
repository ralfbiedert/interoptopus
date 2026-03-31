//! Reverse interop plugin definitions.
use crate::inventory::{Inventory, PluginId, PluginInventory};

/// Signature of the `register_trampoline` function exported by every plugin assembly.
///
/// The first argument is the well-known trampoline ID (see [`crate::trampoline`]);
/// the second is the function or data pointer to register.
pub type RegisterTrampolineFn = extern "C" fn(i64, *const u8);

/// Signature of the `_trampoline_query_u64` function exported by every plugin assembly.
///
/// The argument is a well-known query ID, the return value is the queried `u64` result.
pub type QueryTrampolineFn = extern "C" fn(i64) -> u64;

/// Registers the full type and function surface of a reverse-interop plugin.
///
/// # Safety
///
/// This trait registers the full type and function surface of a reverse-
/// interop plugin. An incorrect `id()` or a `register()` that omits or
/// misrepresents types and functions will cause symbol resolution to target
/// the wrong addresses, leading to undefined behaviour when the plugin is
/// loaded and called across the FFI boundary.
pub unsafe trait PluginInfo {
    /// The unique identifier for this plugin.
    fn id() -> PluginId;

    // fn plugin() -> PluginXXX;

    /// Registers this plugin (and all referenced types) with the given inventory.
    fn register(inventory: &mut impl Inventory);

    /// Returns a [`PluginInventory`] populated with all types, functions, and
    /// services declared by this plugin.
    #[must_use]
    fn inventory() -> PluginInventory {
        let mut inventory = PluginInventory::new();
        Self::register(&mut inventory);
        inventory
    }
}

/// A loaded plugin instance with resolved function pointers.
///
/// Implemented by the `plugin!` macro for each declared plugin struct.
pub trait Plugin: Sized {
    /// Resolves all function pointers using the provided symbol lookup function.
    ///
    /// The `loader` closure takes a symbol name and returns a pointer to the
    /// loaded function, or null if the symbol was not found.
    fn load_from(loader: impl Fn(&str) -> *const u8) -> Result<Self, PluginLoadError>;

    /// Returns the `_trampoline_register` function pointer loaded during [`load_from`](Self::load_from).
    ///
    /// Used by the host to register runtime callbacks (e.g. the uncaught-exception handler)
    /// without needing a second symbol lookup.
    fn register_trampoline_fn(&self) -> RegisterTrampolineFn;

    /// Returns the `_trampoline_query_u64` function pointer loaded during [`load_from`](Self::load_from).
    ///
    /// Used by the host to query plugin metadata (e.g. the API guard hash).
    fn query_trampoline_fn(&self) -> QueryTrampolineFn;

    /// Verifies that the plugin's API hash matches the expected hash.
    ///
    /// Must be called after trampoline registration so the query function works.
    /// The default implementation does nothing; the `plugin!` macro overrides this
    /// with a real check.
    fn verify_api_guard(&self) -> Result<(), PluginLoadError> {
        Ok(())
    }
}

/// A symbol loader bound to a specific assembly/shared library.
///
/// Created by a runtime (e.g., `DotNetRuntime::dll_loader`) after loading
/// an assembly. Resolves symbols and instantiates plugins.
pub trait Loader {
    /// Loads a plugin by resolving its symbols from the bound assembly.
    fn load_plugin<T: Plugin>(&self) -> Result<T, PluginLoadError>;
}

/// Errors that can occur when loading a plugin.
#[derive(Debug)]
pub struct PluginLoadError {
    pub kind: PluginLoadErrorKind,
}

/// The specific kind of plugin load error.
#[derive(Debug)]
pub enum PluginLoadErrorKind {
    /// A required symbol was not found in the loaded assembly.
    SymbolNotFound(String),
    /// The runtime failed to load the assembly.
    LoadFailed(String),
    /// The plugin's API hash does not match the expected hash from the Rust-side declaration.
    ApiMismatch { expected: u64, actual: u64 },
}

impl PluginLoadError {
    /// Creates a `SymbolNotFound` error.
    pub fn symbol_not_found(name: impl Into<String>) -> Self {
        Self { kind: PluginLoadErrorKind::SymbolNotFound(name.into()) }
    }

    /// Creates a `LoadFailed` error.
    pub fn load_failed(msg: impl Into<String>) -> Self {
        Self { kind: PluginLoadErrorKind::LoadFailed(msg.into()) }
    }

    /// Creates an `ApiMismatch` error.
    #[must_use]
    pub fn api_mismatch(expected: u64, actual: u64) -> Self {
        Self { kind: PluginLoadErrorKind::ApiMismatch { expected, actual } }
    }
}

impl std::fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PluginLoadErrorKind::SymbolNotFound(name) => write!(f, "symbol not found: {name}"),
            PluginLoadErrorKind::LoadFailed(msg) => write!(f, "failed to load plugin: {msg}"),
            PluginLoadErrorKind::ApiMismatch { expected, actual } => {
                write!(f, "API mismatch: Rust side expects hash 0x{expected:016x} but plugin reports 0x{actual:016x}. Rebuild the plugin against the current Rust API.")
            }
        }
    }
}

impl std::error::Error for PluginLoadError {}
