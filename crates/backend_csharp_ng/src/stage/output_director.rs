//! Main output configuration.

use crate::dispatch::Dispatch;
use crate::output::{Output, OutputKind};
use crate::template::templates;
use interoptopus::inventory::Inventory;
use interoptopus_backends::template::TemplateEngine;

pub struct Config {
    pub dispatch: Dispatch,
    pub templates: TemplateEngine,
}

impl Default for Config {
    fn default() -> Self {
        Self { dispatch: Default::default(), templates: templates() }
    }
}

pub struct Stage {
    config: Config,
    outputs: Vec<Output>,
}

impl Stage {
    pub fn new(config: Config) -> Self {
        Self { config, outputs: vec![] }
    }

    pub fn process(&mut self, inventory: &Inventory) {
        // TODO: for each possible file, create an entry
        self.outputs.push(Output { name: "Foo.cs".to_string(), kind: OutputKind::Csharp });
    }

    pub fn dispatch(&self) -> &Dispatch {
        &self.config.dispatch
    }

    pub fn templates(&self) -> &TemplateEngine {
        &self.config.templates
    }

    pub fn outputs(&self) -> impl Iterator<Item = &Output> {
        self.outputs.iter()
    }

    pub fn outputs_of(&self, kind: OutputKind) -> impl Iterator<Item = &Output> {
        self.outputs.iter().filter(move |x| x.kind == kind)
    }
}
