//! Reverse interop plugin definitions.
use crate::inventory::{Inventory, PluginId, PluginInventory};

/// Signature of the `register_trampoline` function exported by every plugin assembly.
///
/// The first argument is the well-known trampoline ID (see [`crate::trampoline`]);
/// the second is the function or data pointer to register.
pub type RegisterTrampolineFn = extern "C" fn(i64, *const u8);

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

    /// Returns the `register_trampoline` function pointer loaded during [`load_from`](Self::load_from).
    ///
    /// Used by the host to register runtime callbacks (e.g. the uncaught-exception handler)
    /// without needing a second symbol lookup.
    fn register_trampoline_fn(&self) -> RegisterTrampolineFn;
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
pub enum PluginLoadError {
    /// A required symbol was not found in the loaded assembly.
    SymbolNotFound(String),
    /// The runtime failed to load the assembly.
    LoadFailed(String),
}

impl std::fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SymbolNotFound(name) => write!(f, "symbol not found: {name}"),
            Self::LoadFailed(msg) => write!(f, "failed to load plugin: {msg}"),
        }
    }
}

impl std::error::Error for PluginLoadError {}
