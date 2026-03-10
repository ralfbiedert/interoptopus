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
        Self { info: PassInfo { name: file!() } }
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
            let fns_rust = intermediary.fns_rust.imports_for(file).unwrap();
            let enums = intermediary.enums.enums_for(file).unwrap();
            let composites = intermediary.composites.composites_for(file).unwrap();
            let delegates = intermediary.delegates.delegates_for(file).unwrap();
            let util = intermediary.util.utils_for(file).unwrap();
            let using = intermediary.using.using_for(file).unwrap();

            context.insert("dll_name", meta_info.dll_name());
            context.insert("header", header);
            context.insert("using", using);
            context.insert("fns_rust", &fns_rust);
            context.insert("enums", &enums);
            context.insert("composites", &composites);
            context.insert("delegates", &delegates);
            context.insert("util", &util);

            let final_ = templates.render("all.cs", &context)?;
            output.add_buffer(&file.name, final_);
        }

        Ok(())
    }
}
