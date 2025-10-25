//! Main output configuration.

use crate::dispatch::Dispatch;
use crate::template::templates;
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
use interoptopus_backends::template::TemplateEngine;
use std::marker::PhantomData;

pub struct Config {
    pub dispatch: Dispatch,
    pub templates: TemplateEngine,
    _hidden: PhantomData<()>,
}

impl Default for Config {
    fn default() -> Self {
        Self { dispatch: Default::default(), templates: templates(), _hidden: Default::default() }
    }
}

pub struct Stage {
    config: Config,
}

impl Stage {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn process(&mut self, inventory: &Inventory, output: &mut Multibuf) {
        // TODO: for each possible file, start creating an empty multibuf entry
    }

    pub fn dispatch(&self) -> &Dispatch {
        &self.config.dispatch
    }

    pub fn templates(&self) -> &TemplateEngine {
        &self.config.templates
    }
}
