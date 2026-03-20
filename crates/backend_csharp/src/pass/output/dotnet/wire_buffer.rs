//! Renders the `WireBuffer` utility type for plugin-mode wire transfers.
//!
//! In plugin mode, `WireBuffer.Allocate` and `WireBuffer.Dispose` call through
//! `Trampolines.WireCreate` / `Trampolines.WireDestroy` (function pointers
//! registered by the Rust host) instead of `[LibraryImport]` P/Invoke calls.
//! Only emitted when Wire types are present in the inventory.

use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, output};
use interoptopus::inventory::Types as RsTypes;
use interoptopus::lang::types::TypeKind as RsTypeKind;
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
        rs_types: &RsTypes,
    ) -> OutputResult {
        let templates = output_master.templates();

        // Check if any Wire types exist in the inventory.
        let has_wire = rs_types.values().any(|ty| {
            matches!(&ty.kind, RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Wire(_)))
        });

        for file in output_master.outputs_of(FileType::Csharp) {
            let content = if has_wire {
                let context = Context::new();
                templates.render("dotnet/wire_buffer.cs", &context)?.trim().to_string()
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
