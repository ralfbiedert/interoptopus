//! Renders utility types (exceptions, string extensions) per output file.

use crate::output::{FileType, Output};
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
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, utils: HashMap::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(FileType::Csharp) {
            let mut context = Context::new();

            let interop_exception = templates.render("types/util/interop_exception.cs", &context)?;
            let enum_exception = templates.render("types/util/enum_exception.cs", &context)?;
            let utf8string = templates.render("types/util/utf8string.cs", &context)?;
            let async_callback_common = templates.render("types/util/async_callback_common.cs", &context)?;

            context.insert("interop_exception", &interop_exception.trim());
            context.insert("enum_exception", &enum_exception.trim());
            context.insert("utf8string", &utf8string.trim());
            context.insert("async_callback_common", &async_callback_common.trim());

            let combined = templates.render("types/util/all.cs", &context)?;

            self.utils.insert(output.clone(), combined);
        }
        Ok(())
    }

    #[must_use]
    pub fn utils_for(&self, output: &Output) -> Option<&str> {
        self.utils.get(output).map(|s| &**s)
    }
}
