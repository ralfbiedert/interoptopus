//! Last output step where a buffer is fully materialized.

use crate::output::OutputKind;
use crate::pass::{meta, output, OutputResult, PassInfo};
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
        Self { info: PassInfo { name: "output/final" } }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        meta_info: &meta::info::Pass,
        output: &mut Multibuf,
        output_master: &output::master::Pass,
        intermediary: &IntermediateOutputPasses,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut context = Context::new();

            let header = intermediary.header.header_for(file).unwrap();
            let types = ""; // TODO for now
            let fn_imports = intermediary.fn_imports.imports_for(file).unwrap();
            let enums = intermediary.enums.enums_for(file).unwrap();

            context.insert("dll_name", meta_info.dll_name());
            context.insert("header", header);
            context.insert("types", &types);
            context.insert("fn_imports", &fn_imports);
            context.insert("enums", &enums);

            let final_ = templates.render("final.cs", &context)?;
            output.add_buffer(&file.name, final_);
        }

        Ok(())
    }
}
