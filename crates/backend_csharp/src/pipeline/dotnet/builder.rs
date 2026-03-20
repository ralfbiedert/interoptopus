use super::library::{DotnetLibrary, DotnetLibraryConfig};
use interoptopus::inventory::ForeignInventory;

/// Builder for configuring and constructing a [`DotnetLibrary`].
#[derive(Default)]
pub struct DotnetLibraryBuilder {
    inventory: ForeignInventory,
    config: DotnetLibraryConfig,
}

impl DotnetLibraryBuilder {
    pub(crate) fn new(inventory: ForeignInventory) -> Self {
        Self { inventory, ..Self::default() }
    }

    /// Sets the plugin name used in the generated file and class names.
    #[must_use]
    pub fn plugin_name(mut self, name: impl AsRef<str>) -> Self {
        self.config.plugin_name = name.as_ref().to_string();
        self
    }

    /// Sets the C# namespace for the generated code.
    #[must_use]
    pub fn namespace(mut self, namespace: impl AsRef<str>) -> Self {
        self.config.namespace = namespace.as_ref().to_string();
        self
    }

    /// Builds the configured [`DotnetLibrary`], ready for [`process`](DotnetLibrary::process).
    #[must_use]
    pub fn build(self) -> DotnetLibrary {
        DotnetLibrary::with_config(self.inventory, self.config)
    }
}
