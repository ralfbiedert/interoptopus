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
        meta_info: &meta::rust::info::Pass,
        output: &mut Multibuf,
        output_master: &output::common::master::Pass,
        intermediary: &IntermediateOutputPasses,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut context = Context::new();

            let header = intermediary.header.header_for(file).unwrap();
            let fns_rust = intermediary.fns_rust.imports_for(file).unwrap();
            let api_guard = intermediary.fns_api_guard.api_guard_for(file).unwrap();
            let fns_overload_simple = intermediary.fns_overload_simple.imports_for(file).unwrap();
            let fns_overload_body = intermediary.fns_overload_body.body_imports_for(file).unwrap();
            let fns_overload_asynk = intermediary.fns_overload_body.async_imports_for(file).unwrap();
            let async_trampoline_fields = intermediary.asynk.trampoline_fields_for(file).unwrap();
            let enums = intermediary.enums.enums_for(file).unwrap();
            let composites = intermediary.composites.composites_for(file).unwrap();
            let delegates_class = intermediary.delegates_class.delegates_for(file).unwrap();
            let delegates_signature = intermediary.delegates_signature.delegates_for(file).unwrap();
            let delegates: Vec<&str> = delegates_class.iter().chain(delegates_signature.iter()).map(String::as_str).collect();
            let slices = intermediary.slices.slices_for(file).unwrap();
            let vecs = intermediary.vecs.vecs_for(file).unwrap();
            let services = intermediary.services.services_for(file).unwrap();
            let async_trampolines = intermediary.asynk.trampolines_for(file).unwrap();
            let pattern_bools = intermediary.pattern_bools.bool_for(file).unwrap();
            let pattern_utf8string = intermediary.pattern_utf8string.utf8string_for(file).unwrap();
            let pattern_wire_buffer = intermediary.pattern_wire_buffer.wire_buffer_for(file).unwrap();
            let wires = intermediary.wires.wires_for(file).unwrap();
            let util = intermediary.util.utils_for(file).unwrap();
            let using = intermediary.using.using_for(file).unwrap();

            context.insert("namespace", file.target.namespace());
            context.insert("dll_name", meta_info.dll_name());
            context.insert("header", header);
            context.insert("using", using);
            context.insert("fns_rust", &fns_rust);
            context.insert("api_guard", api_guard);
            context.insert("fns_overload_simple", &fns_overload_simple);
            context.insert("fns_overload_body", &fns_overload_body);
            context.insert("fns_overload_asynk", &fns_overload_asynk);
            context.insert("async_trampoline_fields", &async_trampoline_fields);
            context.insert("enums", &enums);
            context.insert("composites", &composites);
            context.insert("delegates", &delegates);
            context.insert("slices", &slices);
            context.insert("vecs", &vecs);
            context.insert("services", &services);
            context.insert("async_trampolines", &async_trampolines);
            context.insert("pattern_bools", &pattern_bools);
            context.insert("pattern_utf8string", &pattern_utf8string);
            context.insert("pattern_wire_buffer", &pattern_wire_buffer);
            context.insert("wires", &wires);
            context.insert("util", &util);

            let final_ = templates.render("rust/all.cs", &context)?;
            output.add_buffer_with_overwrite(file.target.file_name(), final_, file.target.overwrite_policy());
        }

        Ok(())
    }
}
