//! Renders using directives per output file.

use crate::output::{FileType, Output};
use crate::pass::{output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    usings: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, usings: HashMap::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass) -> OutputResult {
        let templates = output_master.templates();

        let all_outputs: Vec<_> = output_master.outputs_of(FileType::Csharp).collect();

        for output in &all_outputs {
            let mut context = Context::new();

            let this_ns = output.target.namespace();
            let extra_imports: Vec<String> = all_outputs
                .iter()
                .map(|o| o.target.namespace().to_string())
                .filter(|ns| ns != this_ns)
                .collect();

            context.insert("extra_imports", &extra_imports);

            let using = templates.render("using.cs", &context)?;

            self.usings.insert((*output).clone(), using);
        }
        Ok(())
    }

    #[must_use]
    pub fn using_for(&self, output: &Output) -> Option<&str> {
        self.usings.get(output).map(|s| &**s)
    }
}
