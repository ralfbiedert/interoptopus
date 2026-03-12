//! Top-level output configuration.

use crate::dispatch::Dispatch;
use crate::output::{Output, OutputKind};
use crate::pass::{OutputResult, PassInfo};
use crate::template::templates;
use interoptopus_backends::template::TemplateEngine;

pub struct Config {
    pub dispatch: Dispatch,
    pub templates: TemplateEngine,
}

impl Default for Config {
    fn default() -> Self {
        Self { dispatch: Dispatch::default(), templates: templates() }
    }
}

pub struct Pass {
    info: PassInfo,
    config: Config,
    outputs: Vec<Output>,
}

impl Pass {
    #[must_use] 
    pub fn new(config: Config) -> Self {
        Self { info: PassInfo { name: file!() }, config, outputs: vec![] }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta) -> OutputResult {
        // TODO: for each possible file, create an entry
        self.outputs.push(Output { name: "Interop.cs".to_string(), kind: OutputKind::Csharp });
        Ok(())
    }

    #[must_use] 
    pub fn dispatch(&self) -> &Dispatch {
        &self.config.dispatch
    }

    #[must_use] 
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
