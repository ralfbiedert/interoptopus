//! Writes top-level file header.

use crate::output::{Output, OutputKind};
use crate::pass::{meta, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    headers: HashMap<Output, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "output/header" }, headers: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass, meta_info: &meta::info::Pass) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut context = Context::new();

            context.insert("INTEROP_DLL_NAME", meta_info.dll_name());
            context.insert("INTEROP_HASH", meta_info.api_hash());
            context.insert("INTEROP_NAMESPACE", "TODO");
            context.insert("INTEROPTOPUS_CRATE", meta_info.interoptopus_crate());
            context.insert("INTEROPTOPUS_VERSION", meta_info.interoptopus_version());

            let header = templates.render("header.cs", &context)?;

            self.headers.insert(output.clone(), header);
        }
        Ok(())
    }

    pub fn header_for(&self, output: &Output) -> Option<&str> {
        self.headers.get(output).map(|s| &**s)
    }
}
