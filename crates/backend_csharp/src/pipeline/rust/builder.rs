use crate::dispatch::Dispatch;
use crate::{RustLibrary, RustLibraryConfig};
use interoptopus::inventory::RustInventory;
use interoptopus_backends::template::TemplateEngine;

/// Builder for configuring and constructing a [`RustLibrary`].
#[derive(Default)]
pub struct RustLibraryBuilder {
    inventory: RustInventory,
    config: RustLibraryConfig,
}

impl RustLibraryBuilder {
    pub(crate) fn new(inventory: RustInventory) -> Self {
        Self { inventory, ..Self::default() }
    }

    /// Sets the dispatch strategy that routes items to output files.
    #[must_use]
    pub fn dispatch(mut self, dispatch: Dispatch) -> Self {
        self.config.output_master.dispatch = dispatch;
        self
    }

    /// Sets the native library name used in `[DllImport("...")]` declarations.
    #[must_use]
    pub fn dll_name(mut self, dll_name: impl AsRef<str>) -> Self {
        self.config.meta_info.dll_name = dll_name.as_ref().to_string();
        self
    }

    /// Allows users to specify custom templates for code generation.
    ///
    /// Currently not exposed as we'd implicitly stabilize a huge template definition
    /// surface.
    fn templates(mut self, templates: TemplateEngine) -> Self {
        self.config.output_master.templates = templates;
        self
    }

    /// Builds the configured [`RustLibrary`], ready for [`process`](RustLibrary::process).
    #[must_use]
    pub fn build(self) -> RustLibrary {
        RustLibrary::with_config(self.inventory, self.config)
    }
}
