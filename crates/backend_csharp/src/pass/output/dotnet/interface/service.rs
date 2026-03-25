//! Renders service interfaces (e.g. `IFoo<TSelf>`) with method declarations.
//!
//! ```csharp
//! public interface IFoo<TSelf> where TSelf : IFoo<TSelf>
//! {
//!     static abstract TSelf Create();
//!     void Bar(int x);
//!     int GetAccumulator();
//! }
//! ```
//!
//! Uses the service interface model pass for method names and signatures.

use crate::dispatch::{Item, ItemKind};
use crate::lang::plugin::interface::MethodKind;
use crate::output::{FileType, Output};
use crate::pass::output::dotnet::interface::{format_args, rval_display_name};
use crate::pass::{OutputResult, PassInfo, model, output};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interfaces: HashMap<Output, Vec<String>>,
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
        service_interfaces: &model::dotnet::interface::service::Pass,
        types: &model::common::types::all::Pass,
    ) -> OutputResult {
        for file in output_master.outputs_of(FileType::Csharp) {
            let mut all_interfaces = Vec::new();

            for interface in service_interfaces.interfaces() {
                let Some(fe) = interface.emission.file_emission() else { continue };
                if !output_master.item_belongs_to(Item { kind: ItemKind::PluginInterface, emission: fe.clone() }, file) {
                    continue;
                }

                let mut members = Vec::new();

                for method in &interface.methods {
                    let args_str = format_args(&method.csharp.arguments, types);

                    let line = match method.kind {
                        MethodKind::Static => {
                            format!("    static abstract TSelf {}({args_str});", method.name)
                        }
                        MethodKind::Regular => {
                            let rval_name = rval_display_name(method, types);
                            format!("    {} {}({args_str});", rval_name, method.name)
                        }
                    };

                    members.push(line);
                }

                let body = members.join("\n");
                let rendered = format!("public interface {}<TSelf> where TSelf : {}<TSelf>\n{{\n{body}\n}}", interface.name, interface.name);

                all_interfaces.push(rendered);
            }

            self.interfaces.insert(file.clone(), all_interfaces);
        }

        Ok(())
    }

    #[must_use]
    pub fn interfaces_for(&self, output: &Output) -> Option<&[String]> {
        self.interfaces.get(output).map(Vec::as_slice)
    }
}
