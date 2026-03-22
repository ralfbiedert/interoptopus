//! Reverse interop plugin definitions.
use crate::inventory::{ForeignInventory, Inventory, PluginId};

pub trait PluginInfo {
    /// The unique identifier for this plugin.
    fn id() -> PluginId;

    // fn plugin() -> PluginXXX;

    /// Registers this plugin (and all referenced types) with the given inventory.
    fn register(inventory: &mut impl Inventory);

    /// Returns a [`ForeignInventory`] populated with all types, functions, and
    /// services declared by this plugin.
    #[must_use]
    fn inventory() -> ForeignInventory {
        let mut inventory = ForeignInventory::new();
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
