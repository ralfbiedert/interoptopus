//! Last output step where a buffer is fully materialized.

use crate::output::FileType;
use crate::pass::{OutputResult, PassInfo, meta, output};
use crate::pipeline::IntermediateOutputPasses;
use interoptopus_backends::output::Multibuf;
use interoptopus_backends::template::Context;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    #[must_use]
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

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut context = Context::new();

            let header = intermediary.header.header_for(file).unwrap();
            let fns_rust = intermediary.fns_rust.imports_for(file).unwrap();
            let fns_overload_simple = intermediary.fns_overload_simple.imports_for(file).unwrap();
            let fns_overload_body = intermediary.fns_overload_body.body_imports_for(file).unwrap();
            let fns_overload_asynk = intermediary.fns_overload_body.async_imports_for(file).unwrap();
            let async_trampoline_fields = intermediary.asynk.trampoline_fields_for(file).unwrap();
            let enums = intermediary.enums.enums_for(file).unwrap();
            let composites = intermediary.composites.composites_for(file).unwrap();
            let delegates = intermediary.delegates.delegates_for(file).unwrap();
            let slices = intermediary.slices.slices_for(file).unwrap();
            let services = intermediary.services.services_for(file).unwrap();
            let async_trampolines = intermediary.asynk.trampolines_for(file).unwrap();
            let util = intermediary.util.utils_for(file).unwrap();
            let using = intermediary.using.using_for(file).unwrap();

            context.insert("namespace", file.target.namespace());
            context.insert("dll_name", meta_info.dll_name());
            context.insert("header", header);
            context.insert("using", using);
            context.insert("fns_rust", &fns_rust);
            context.insert("fns_overload_simple", &fns_overload_simple);
            context.insert("fns_overload_body", &fns_overload_body);
            context.insert("fns_overload_asynk", &fns_overload_asynk);
            context.insert("async_trampoline_fields", &async_trampoline_fields);
            context.insert("enums", &enums);
            context.insert("composites", &composites);
            context.insert("delegates", &delegates);
            context.insert("slices", &slices);
            context.insert("services", &services);
            context.insert("async_trampolines", &async_trampolines);
            context.insert("util", &util);

            let final_ = templates.render("all.cs", &context)?;
            output.add_buffer(file.target.file_name(), final_);
        }

        Ok(())
    }
}
