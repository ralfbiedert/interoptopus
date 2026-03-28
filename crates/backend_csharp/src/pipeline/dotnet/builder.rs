use super::library::{DotnetLibrary, DotnetLibraryConfig};
use crate::dispatch::Dispatch;
use crate::lang::plugin::PLUGIN_DEFAULT_MODULE;
use crate::output::Target;
use crate::pass::output;
use crate::pattern::Exception;
use interoptopus::inventory::PluginInventory;
use interoptopus::lang::meta::FileEmission;

/// Builder for configuring and constructing a [`DotnetLibrary`].
#[derive(Default)]
pub struct DotnetLibraryBuilder {
    inventory: PluginInventory,
    config: DotnetLibraryConfig,
}

impl DotnetLibraryBuilder {
    pub(crate) fn new(inventory: PluginInventory) -> Self {
        let default_dispatch = Dispatch::custom(|x, _| match x.emission {
            FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
            FileEmission::Default => Target::new("Interop.User.cs", "My.Company"),
            FileEmission::CustomModule(m) if m == PLUGIN_DEFAULT_MODULE => Target::new("Interop.Plugin.cs", "Interoptopus.API"),
            FileEmission::CustomModule(_) => Target::new("Interop.User.cs", "My.Company"),
        });

        let config = DotnetLibraryConfig { output_master: output::common::master::Config { dispatch: default_dispatch, ..Default::default() }, ..Default::default() };

        Self { inventory, config }
    }

    /// Sets the dispatch strategy that routes items to output files.
    #[must_use]
    pub fn dispatch(mut self, dispatch: Dispatch) -> Self {
        self.config.output_master.dispatch = dispatch;
        self
    }

    /// Registers a named C# exception type for structured error mapping in `FromCall`.
    #[must_use]
    pub fn exception(mut self, exception: Exception) -> Self {
        self.config.model_exceptions.exceptions.push(exception);
        self
    }

    /// Builds the configured [`DotnetLibrary`], ready for [`process`](DotnetLibrary::process).
    #[must_use]
    pub fn build(self) -> DotnetLibrary {
        DotnetLibrary::with_config(self.inventory, self.config)
    }
}
