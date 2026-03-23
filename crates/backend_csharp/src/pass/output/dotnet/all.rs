//! Renders the final dotnet interop `.cs` file by composing intermediary output passes.

use crate::output::FileType;
use crate::pass::{OutputResult, PassInfo, output};
use crate::pipeline::DotnetOutputPasses;
use interoptopus_backends::output::Multibuf;
use interoptopus_backends::template::Context;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    #[must_use]
    pub fn new(_config: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        intermediary: &DotnetOutputPasses,
        output: &mut Multibuf,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut context = Context::new();

            let plugin_interface = intermediary.plugin_interface.interface_for(file).unwrap_or("");
            let service_interfaces = intermediary.service_interface.interfaces_for(file).unwrap_or(&[]);
            let trampolines = intermediary.interop.trampolines_for(file).unwrap_or(&[]);
            let delegates_class = intermediary.delegates_class.delegates_for(file).unwrap_or(&[]);
            let delegates_signature = intermediary.delegates_signature.delegates_for(file).unwrap_or(&[]);
            let delegates: Vec<&str> = delegates_class.iter().chain(delegates_signature.iter()).map(String::as_str).collect();
            let composites = intermediary.composites.composites_for(file).unwrap_or(&[]);
            let enums = intermediary.enums.enums_for(file).unwrap_or(&[]);
            let pattern_bools = intermediary.pattern_bools.bool_for(file).unwrap_or("");
            let util = intermediary.util.utils_for(file).unwrap_or("");
            let trampoline_class = intermediary.trampoline.trampoline_for(file).unwrap_or("");
            let wire_buffer = intermediary.wire_buffer.wire_buffer_for(file).unwrap_or("");
            let wires = intermediary.wires.wires_for(file).unwrap_or(&[]);

            let usings = intermediary.using.using_for(file).unwrap_or("");

            context.insert("usings", usings);
            context.insert("namespace", file.target.namespace());
            context.insert("delegates", &delegates);
            context.insert("composites", &composites);
            context.insert("enums", &enums);
            context.insert("pattern_bools", &pattern_bools);
            context.insert("util", &util);
            context.insert("trampoline_class", trampoline_class);
            context.insert("wire_buffer", wire_buffer);
            context.insert("wires", &wires);
            context.insert("plugin_interface", plugin_interface);
            context.insert("service_interfaces", &service_interfaces);
            context.insert("trampolines", &trampolines);

            let rendered = templates.render("dotnet/all.cs", &context)?;
            output.add_buffer_with_overwrite(file.target.file_name(), rendered, file.target.overwrite_policy());
        }

        Ok(())
    }
}
