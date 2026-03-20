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
            let trampolines = intermediary.trampoline.trampolines_for(file).unwrap_or(&[]);
            let composites = intermediary.composites.composites_for(file).unwrap_or(&[]);

            context.insert("namespace", file.target.namespace());
            context.insert("composites", &composites);
            context.insert("plugin_interface", plugin_interface);
            context.insert("service_interfaces", &service_interfaces);
            context.insert("trampolines", &trampolines);

            let rendered = templates.render("dotnet/all.cs", &context)?;
            output.add_buffer(file.target.file_name(), rendered);
        }

        Ok(())
    }
}
