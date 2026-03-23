//! Collects all `[UnmanagedCallersOnly]` interop methods (raw + service) into a
//! single list per output file, preserving the original trampoline entry order.
//!
//! The `register_trampoline` entry point is always emitted first so that the Rust
//! host can register runtime callbacks. Then raw function trampolines and service
//! trampolines follow in their original declaration order.
//!
//! All interop methods are routed to the plugin interface's output file via
//! `item_belongs_to`, not to the original function's output file.

use crate::dispatch::{Item, ItemKind};
use crate::lang::plugin::PLUGIN_DEFAULT_EMISSION;
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use crate::pass::output::dotnet::interop::{raw, service};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    trampolines: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, trampolines: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        trampoline_model: &model::dotnet::trampoline::Pass,
        raw_pass: &raw::Pass,
        service_pass: &service::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            // All interop methods belong to the plugin output file.
            let emission = PLUGIN_DEFAULT_EMISSION;
            let Some(fe) = emission.file_emission() else {
                self.trampolines.insert(file.clone(), Vec::new());
                continue;
            };
            if !output_master.item_belongs_to(Item { kind: ItemKind::PluginInterface, emission: fe.clone() }, file) {
                self.trampolines.insert(file.clone(), Vec::new());
                continue;
            }

            let mut methods = Vec::new();

            for entry in trampoline_model.entries() {
                if let Some(m) = raw_pass.get(entry.fn_id) {
                    methods.push(m.to_string());
                } else if let Some(m) = service_pass.get(entry.fn_id) {
                    methods.push(m.to_string());
                }
            }

            if !methods.is_empty() {
                let ctx = Context::new();
                let register = templates.render("dotnet/interop/register_trampoline.cs", &ctx)?;
                methods.insert(0, register.trim_end().to_string());
            }

            self.trampolines.insert(file.clone(), methods);
        }

        Ok(())
    }

    #[must_use]
    pub fn trampolines_for(&self, output: &Output) -> Option<&[String]> {
        self.trampolines.get(output).map(Vec::as_slice)
    }
}
