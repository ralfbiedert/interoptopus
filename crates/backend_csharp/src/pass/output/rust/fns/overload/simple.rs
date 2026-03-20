//! Writes function import declarations for simple overloads.
//!
//! Simple overloads do not contain a body — they are plain `DllImport` declarations.
//! The overload functions are identified via their `FunctionKind::Overload` with
//! `OverloadKind::Simple`, queried from the central `fns::all` pass.

use crate::lang::functions::FunctionKind;
use crate::lang::functions::overload::OverloadKind;
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    fn_imports: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, fn_imports: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(FileType::Csharp) {
            let mut imports = Vec::new();

            for (&overload_id, function) in fns_all.overloads() {
                // Only simple overloads get rendered here
                let FunctionKind::Overload(ref overload) = function.kind else { continue };
                if !matches!(overload.kind, OverloadKind::Simple) {
                    continue;
                }

                if !output_master.fn_belongs_to(overload.base, output) {
                    continue;
                }

                let name = &function.name;
                let rval = types
                    .get(function.signature.rval)
                    .map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("rval of overload `{name}`")))?;

                let mut args: Vec<HashMap<&str, String>> = Vec::new();
                for arg in &function.signature.arguments {
                    let arg_type = types
                        .get(arg.ty)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("arg `{}` of overload `{}`", arg.name, name)))?;
                    let mut m = HashMap::new();
                    m.insert("name", arg.name.clone());
                    let decorated = match &arg_type.decorators.param {
                        Some(d) => format!("{d} {}", arg_type.name),
                        None => arg_type.name.clone(),
                    };
                    m.insert("ty", decorated);
                    args.push(m);
                }

                let mut context = Context::new();

                context.insert("name", name);
                context.insert("symbol", name);
                context.insert("args", &args);
                context.insert("rval", rval);

                let import = templates.render("rust/fns/overload/simple.cs", &context)?;
                imports.push(import);
            }

            imports.sort();

            self.fn_imports.insert(output.clone(), imports);
        }

        Ok(())
    }

    #[must_use]
    pub fn imports_for(&self, output: &Output) -> Option<&[String]> {
        self.fn_imports.get(output).map(std::vec::Vec::as_slice)
    }
}
