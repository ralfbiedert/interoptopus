//! Writes function import declarations.

use crate::output::{Output, OutputKind};
use crate::pass::{model_fn_map, output_master, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    fn_imports: HashMap<Output, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "output_fn_imports" }, fn_imports: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut super::PassMeta, output_master: &output_master::Pass, fn_maps: &model_fn_map::Pass) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut imports = Vec::new();

            for (id, function) in fn_maps.iter() {
                let name = &function.name;
                for overload in function.overloads {
                    let mut context = Context::new();

                    context.insert("name", name);

                    let import = templates.render("fn_import.cs", &context)?;
                    imports.push(import);
                }
            }

            self.fn_imports.insert(output.clone(), imports);
        }

        Ok(())
    }

    pub fn imports_for(&self, output: &Output) -> Option<&[String]> {
        self.fn_imports.get(output).map(|s| s.as_slice())
    }
}
