//! Writes top-level file header.

use crate::output::{Output, OutputKind};
use crate::stage::{ProcessError, meta_info, output_master};
use interoptopus::inventory::Inventory;
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Stage {
    headers: HashMap<Output, String>,
}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self { headers: Default::default() }
    }

    pub fn process(&mut self, _: &Inventory, output_master: &output_master::Stage, meta_info: &meta_info::Stage) -> ProcessError {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut context = Context::new();

            context.insert("INTEROP_DLL_NAME", meta_info.interop_dll_name());
            context.insert("INTEROP_HASH", meta_info.interop_hash());
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
