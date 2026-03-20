//! Renders the `IPlugin` interface containing static abstract declarations for
//! all raw (non-service) functions.
//!
//! ```csharp
//! public interface IPlugin
//! {
//!     static abstract long DoMath(long a, long b);
//! }
//! ```

use crate::lang::plugin::TrampolineKind;
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::casing::rust_to_pascal;
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
        trampoline_model: &model::dotnet::trampoline::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
    ) -> OutputResult {
        for file in output_master.outputs_of(FileType::Csharp) {
            let mut members = Vec::new();

            for entry in trampoline_model.entries() {
                if !matches!(entry.kind, TrampolineKind::Raw) {
                    continue;
                }

                let Some(func) = fns_all.get(entry.fn_id) else { continue };

                let rval_name = types.get(func.signature.rval).map(|t| t.name.as_str()).unwrap_or("void");
                let pascal_name = rust_to_pascal(&func.name);

                let args: Vec<String> = func
                    .signature
                    .arguments
                    .iter()
                    .filter_map(|arg| {
                        let ty_name = types.get(arg.ty).map(|t| &t.name)?;
                        Some(format!("{} {}", ty_name, arg.name))
                    })
                    .collect();
                let args_str = args.join(", ");

                members.push(format!("    static abstract {rval_name} {pascal_name}({args_str});"));
            }

            if members.is_empty() {
                self.interfaces.insert(file.clone(), String::new());
                continue;
            }

            let body = members.join("\n");
            let rendered = format!("public interface IPlugin\n{{\n{body}\n}}");

            self.interfaces.insert(file.clone(), rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn interface_for(&self, output: &Output) -> Option<&str> {
        self.interfaces.get(output).map(String::as_str)
    }
}
