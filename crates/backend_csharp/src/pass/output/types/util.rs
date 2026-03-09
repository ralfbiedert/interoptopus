//! Renders utility types (exceptions, string extensions) per output file.

use crate::output::{Output, OutputKind};
use crate::pass::{output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    utils: HashMap<Output, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, utils: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut context = Context::new();

            let interop_exception = templates.render("types/util/interop_exception.cs", &context)?;
            let enum_exception = templates.render("types/util/enum_exception.cs", &context)?;
            let utf8string = templates.render("types/util/utf8string.cs", &context)?;

            context.insert("interop_exception", &interop_exception.trim());
            context.insert("enum_exception", &enum_exception.trim());
            context.insert("utf8string", &utf8string.trim());

            let combined = templates.render("types/util/all.cs", &context)?;

            self.utils.insert(output.clone(), combined);
        }
        Ok(())
    }

    pub fn utils_for(&self, output: &Output) -> Option<&str> {
        self.utils.get(output).map(|s| &**s)
    }
}
