//! Renders the `Trampolines` static class for reverse-interop plugin scenarios.
//!
//! The `Trampolines` class stores function pointers registered at load time by
//! the Rust host. It exposes a `register_trampoline(id, fn_ptr)` entrypoint
//! (`[UnmanagedCallersOnly]`) that the host calls to register runtime callbacks
//! such as wire buffer allocation and deallocation.
//!
//! This is the plugin-mode counterpart of the `WireInterop` class used in the
//! Rust-library pipeline (which uses `[LibraryImport]` to call into the native
//! DLL directly).

use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, output};
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
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let context = Context::new();
            let content = templates.render("dotnet/trampolines.cs", &context)?.trim().to_string();
            self.rendered.insert(file.clone(), content);
        }

        Ok(())
    }

    #[must_use]
    pub fn trampolines_for(&self, output: &Output) -> Option<&str> {
        self.rendered.get(output).map(|s| &**s)
    }
}
