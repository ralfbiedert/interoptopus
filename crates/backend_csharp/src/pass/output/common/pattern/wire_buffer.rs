//! Renders the `WireBuffer` utility type for serialized `Wire<T>` transfers.
//!
//! The `WireBuffer` is a composite struct matching the Rust `WireBuffer`
//! layout (`IntPtr` + int + int). It provides managed/unmanaged conversion
//! and a marshaller, following the same pattern as other composite types.
//!
//! When the wire helpers pass discovers `builtins_wire!()` functions in the
//! inventory (Rust-library mode), `Allocate` and `Dispose` call
//! `WireInterop.interoptopus_wire_create/destroy` via `[LibraryImport]`.
//!
//! When Wire types are present but no helper functions are found (plugin mode),
//! they call `Trampoline.WireCreate/WireDestroy` (function pointers registered
//! by the Rust host at load time).

use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus::inventory::{Functions, Types as RsTypes};
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
        output_master: &output::common::master::Pass,
        wire_helpers: &model::common::wire::helpers::Pass,
        rs_functions: &Functions,
        rs_types: &RsTypes,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let content = if let Some(h) = wire_helpers.helpers() {
                // Rust mode: builtins_wire!() found — use DllImport entry points.
                let create_name = &rs_functions[&h.create_fn].name;
                let destroy_name = &rs_functions[&h.destroy_fn].name;

                let mut context = Context::new();
                context.insert("create_entry_point", create_name);
                context.insert("destroy_entry_point", destroy_name);
                context.insert("plugin_mode", &false);
                templates.render("common/pattern/wire_buffer.cs", &context)?.trim().to_string()
            } else if has_wire_types(rs_types) {
                // Plugin mode: no helper functions, but Wire types present — use Trampoline.
                let mut context = Context::new();
                context.insert("plugin_mode", &true);
                context.insert("create_entry_point", "");
                context.insert("destroy_entry_point", "");
                templates.render("common/pattern/wire_buffer.cs", &context)?.trim().to_string()
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

fn has_wire_types(rs_types: &RsTypes) -> bool {
    rs_types
        .values()
        .any(|ty| matches!(&ty.kind, interoptopus::lang::types::TypeKind::TypePattern(interoptopus::lang::types::TypePattern::Wire(_))))
}
