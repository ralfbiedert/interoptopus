//! Last output step where a buffer is fully materialized.

use crate::output::OutputKind;
use crate::pass::{OutputResult, PassInfo, output_master};
use crate::pipeline::IntermediateOutputPasses;
use interoptopus_backends::output::Multibuf;
use interoptopus_backends::template::Context;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: "output_final" },
        }
    }

    pub fn process(&mut self, _pass_meta: &mut super::PassMeta, output: &mut Multibuf, output_master: &output_master::Pass, intermediary: &IntermediateOutputPasses) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut context = Context::new();

            let header = intermediary.header.header_for(file).unwrap();
            let types = ""; // TODO for now

            context.insert("header", header);
            context.insert("types", &types);

            let final_ = templates.render("final.cs", &context)?;
            output.add_buffer(&file.name, final_);
        }

        Ok(())
    }
}
