use super::library::{DotnetLibrary, DotnetLibraryConfig};
use crate::dispatch::Dispatch;
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

    /// Sets the dispatch strategy that routes items to output files.
    #[must_use]
    pub fn dispatch(mut self, dispatch: Dispatch) -> Self {
        self.config.output_master.dispatch = dispatch;
        self
    }

    /// Builds the configured [`DotnetLibrary`], ready for [`process`](DotnetLibrary::process).
    #[must_use]
    pub fn build(self) -> DotnetLibrary {
        DotnetLibrary::with_config(self.inventory, self.config)
    }
}
