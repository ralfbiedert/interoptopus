use crate::dispatch::Dispatch;
use crate::{RustLibrary, RustLibraryConfig};
use interoptopus::inventory::RustInventory;
use interoptopus_backends::template::TemplateEngine;

#[derive(Default)]
pub struct RustLibraryBuilder {
    inventory: RustInventory,
    config: RustLibraryConfig,
}

impl RustLibraryBuilder {
    pub(crate) fn new(inventory: RustInventory) -> Self {
        Self { inventory, ..Self::default() }
    }

    pub fn dispatch(mut self, dispatch: Dispatch) -> Self {
        self.config.output_master.dispatch = dispatch;
        self
    }

    pub fn dll_name(mut self, dll_name: impl AsRef<str>) -> Self {
        self.config.meta_info.dll_name = dll_name.as_ref().to_string();
        self
    }

    pub fn templates(mut self, templates: TemplateEngine) -> Self {
        self.config.output_master.templates = templates;
        self
    }

    pub fn build(self) -> RustLibrary {
        RustLibrary::with_config(self.inventory, self.config)
    }
}
