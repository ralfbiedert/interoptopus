//! Renders the `IPlugin` interface containing static abstract declarations for
//! all raw (non-service) functions.
//!
//! ```csharp
//! public interface IPlugin
//! {
//!     static abstract long DoMath(long a, long b);
//! }
//! ```
//!
//! Uses the plugin interface model pass for method names and signatures.

use crate::dispatch::{Item, ItemKind};
use crate::output::{FileType, Output};
use crate::pass::output::dotnet::interface::{format_args, rval_display_name};
use crate::pass::{OutputResult, PassInfo, model, output};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interfaces: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, interfaces: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        plugin_interface: &model::dotnet::interface::plugin::Pass,
        types: &model::common::types::all::Pass,
    ) -> OutputResult {
        let Some(interface) = plugin_interface.interface() else {
            for file in output_master.outputs_of(FileType::Csharp) {
                self.interfaces.insert(file.clone(), String::new());
            }
            return Ok(());
        };

        for file in output_master.outputs_of(FileType::Csharp) {
            let Some(fe) = interface.emission.file_emission() else { continue };
            if !output_master.item_belongs_to(Item { kind: ItemKind::PluginInterface, emission: fe.clone() }, file) {
                self.interfaces.insert(file.clone(), String::new());
                continue;
            }

            let mut members = Vec::new();

            for method in &interface.methods {
                let args_str = format_args(&method.csharp.arguments, types);
                let rval_name = rval_display_name(method, types);
                members.push(format!("    static abstract {rval_name} {}({args_str});", method.name));
            }

            if members.is_empty() {
                self.interfaces.insert(file.clone(), String::new());
                continue;
            }

            let body = members.join("\n");
            let rendered = format!("public interface {}\n{{\n{body}\n}}", interface.name);
            self.interfaces.insert(file.clone(), rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn interface_for(&self, output: &Output) -> Option<&str> {
        self.interfaces.get(output).map(String::as_str)
    }
}
