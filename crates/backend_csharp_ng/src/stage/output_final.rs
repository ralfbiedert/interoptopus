//! Last output step where a buffer is fully materialized.

use crate::output::OutputKind;
use crate::pipeline::IntermediateOutputStages;
use crate::stage::{ProcessError, output_master};
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
use interoptopus_backends::template::Context;

#[derive(Default)]
pub struct Config {}

pub struct Stage {}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self, _: &Inventory, output: &mut Multibuf, output_master: &output_master::Stage, intermediary: &IntermediateOutputStages) -> ProcessError {
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
