use crate::dispatch::Dispatch;
use crate::template::templates;
use crate::{RustPlugin, RustPluginConfig};
use interoptopus::inventory::Inventory;
use interoptopus_backends::template::TemplateEngine;

#[derive(Default)]
pub struct RustPluginBuilder {
    inventory: Inventory,
    config: RustPluginConfig,
}

impl RustPluginBuilder {
    pub(crate) fn new(inventory: Inventory) -> Self {
        Self { inventory, ..Default::default() }
    }

    pub fn dispatch(mut self, dispatch: Dispatch) -> Self {
        self.config.output_director.dispatch = dispatch;
        self
    }

    pub fn templates(mut self, templates: TemplateEngine) -> Self {
        self.config.output_director.templates = templates;
        self
    }

    pub fn build(self) -> RustPlugin {
        RustPlugin::with_config(self.inventory, self.config)
    }
}
