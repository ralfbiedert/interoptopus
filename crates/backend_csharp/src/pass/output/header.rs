//! Writes top-level file header.

use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, meta, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

/// Configuration for the header output pass.
#[derive(Debug, Clone)]
pub struct Config {
    /// If `true` (default), includes the interoptopus crate version in the
    /// `Builder:` line of the generated file header. Set to `false` to produce
    /// headers that don't change across crate version bumps.
    pub emit_version: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { emit_version: true }
    }
}

pub struct Pass {
    info: PassInfo,
    config: Config,
    headers: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { info: PassInfo { name: file!() }, config, headers: HashMap::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass, meta_info: &meta::info::Pass) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(FileType::Csharp) {
            let mut context = Context::new();

            context.insert("INTEROP_DLL_NAME", meta_info.dll_name());
            context.insert("INTEROP_HASH", meta_info.api_hash());
            context.insert("INTEROP_NAMESPACE", output.target.namespace());
            context.insert("INTEROPTOPUS_CRATE", meta_info.interoptopus_crate());
            context.insert("INTEROPTOPUS_VERSION", meta_info.interoptopus_version());
            context.insert("emit_version", &self.config.emit_version);

            let header = templates.render("header.cs", &context)?;

            self.headers.insert(output.clone(), header);
        }
        Ok(())
    }

    #[must_use]
    pub fn header_for(&self, output: &Output) -> Option<&str> {
        self.headers.get(output).map(|s| &**s)
    }
}
