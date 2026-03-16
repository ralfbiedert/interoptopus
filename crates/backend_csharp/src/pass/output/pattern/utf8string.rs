//! Renders the `Utf8String` pattern type (managed wrapper + unmanaged struct).
//!
//! The `Utf8String` type is a pattern type (`TypePattern::Utf8String`) that gets
//! its own dedicated C# class with marshalling, disposal, and conversion logic.
//! This pass renders it from the `pattern/utf8string.cs` template for each
//! output file that contains the type, using the helper function entry points
//! discovered by the `model::pattern::string` pass.

use crate::lang::types::csharp;
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    rendered: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, rendered: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        pattern_string: &model::pattern::string::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let content = if output_master.type_belongs_to(csharp::UTF8_STRING, file) {
                let helpers = pattern_string.helpers();

                let mut context = Context::new();
                if let Some(h) = helpers {
                    context.insert("create_entry_point", &h.create_entry_point);
                    context.insert("destroy_entry_point", &h.destroy_entry_point);
                    context.insert("clone_entry_point", &h.clone_entry_point);
                }

                templates.render("pattern/utf8string.cs", &context)?.trim().to_string()
            } else {
                String::new()
            };

            self.rendered.insert(file.clone(), content);
        }

        Ok(())
    }

    #[must_use]
    pub fn utf8string_for(&self, output: &Output) -> Option<&str> {
        self.rendered.get(output).map(|s| &**s)
    }
}
