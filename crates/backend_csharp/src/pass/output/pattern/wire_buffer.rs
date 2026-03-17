//! Renders the `WireBuffer` utility type for serialized `Wire<T>` transfers.
//!
//! The `WireBuffer` is a composite struct matching the Rust `WireBuffer`
//! layout (`IntPtr` + int + int). It provides managed/unmanaged conversion
//! and a marshaller, following the same pattern as other composite types.
//! Only emitted when `builtins_wire!()` is registered.

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

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass, pattern_wire: &model::pattern::wire::Pass) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let content = if output_master.type_belongs_to(csharp::UTIL_WIRE_BUFFER, file) {
                if let Some(h) = pattern_wire.helpers() {
                    let mut context = Context::new();
                    context.insert("destroy_entry_point", &h.destroy_entry_point);
                    templates.render("pattern/wire_buffer.cs", &context)?.trim().to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            self.rendered.insert(file.clone(), content);
        }

        Ok(())
    }

    #[must_use]
    pub fn wire_buffer_for(&self, output: &Output) -> Option<&str> {
        self.rendered.get(output).map(|s| &**s)
    }
}
