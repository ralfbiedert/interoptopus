//! Renders the final plugin interop `.cs` file for a .NET plugin (reverse interop).

use crate::output::FileType;
use crate::pass::{OutputResult, PassInfo, output};
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
        Self {
            info: PassInfo { name: file!() },
        }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        output: &mut Multibuf,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut context = Context::new();

            context.insert("namespace", file.target.namespace());

            let rendered = templates.render("plugin/all.cs", &context)?;
            output.add_buffer(file.target.file_name(), rendered);
        }

        Ok(())
    }
}
