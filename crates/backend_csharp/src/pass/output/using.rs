//! Renders using directives per output file.

use crate::output::{Output, OutputKind};
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
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, usings: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut context = Context::new();

            let extra_imports: Vec<String> = vec![];
            context.insert("extra_imports", &extra_imports);

            let using = templates.render("using.cs", &context)?;

            self.usings.insert(output.clone(), using);
        }
        Ok(())
    }

    pub fn using_for(&self, output: &Output) -> Option<&str> {
        self.usings.get(output).map(|s| &**s)
    }
}
