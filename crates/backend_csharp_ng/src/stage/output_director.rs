//! Main output configuration.

use crate::dispatch::Dispatch;
use crate::template::templates;
use interoptopus::inventory::Inventory;
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

pub struct Stage {}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self, inventory: &Inventory) {}
}
