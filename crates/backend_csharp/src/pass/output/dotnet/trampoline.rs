//! Renders the `Trampoline` static class for reverse-interop plugin scenarios.
//!
//! The `Trampoline` class stores function pointers registered at load time by
//! the Rust host. It exposes a `register_trampoline(id, fn_ptr)` entrypoint
//! (`[UnmanagedCallersOnly]`) that the host calls to register runtime callbacks
//! such as wire buffer allocation and deallocation.
//!
//! This is the plugin-mode counterpart of the `WireInterop` class used in the
//! Rust-library pipeline (which uses `[LibraryImport]` to call into the native
//! DLL directly).
//!
//! Only emitted into output files that contain at least one trampoline function.

use crate::output::{FileType, Output};
use crate::pass::output::dotnet::interop;
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

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::common::master::Pass, interop_pass: &interop::all::Pass) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let has_trampolines = interop_pass.trampolines_for(file).is_some_and(|t| !t.is_empty());

            let content = if has_trampolines {
                let ctx = Context::new();
                templates.render("dotnet/trampoline.cs", &ctx)?.trim().to_string()
            } else {
                String::new()
            };

            self.rendered.insert(file.clone(), content);
        }

        Ok(())
    }

    #[must_use]
    pub fn trampoline_for(&self, output: &Output) -> Option<&str> {
        self.rendered.get(output).map(|s| &**s)
    }
}
