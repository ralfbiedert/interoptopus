//! Writes function import declarations for simple overloads.
//!
//! Simple overloads do not contain a body — they are plain `DllImport` declarations.
//! The overload IDs come from the simple model pass, with the actual Function
//! objects looked up from `fns::all`.

use crate::output::{FileType, Output};
use crate::pass::{model, output, OutputResult, PassInfo};
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
        output_master: &output::master::Pass,
        overload_simple: &model::fns::overload::simple::Pass,
        fn_all: &model::fns::all::Pass,
        types: &model::types::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(FileType::Csharp) {
            let mut imports = Vec::new();

            for overload_id in overload_simple.iter() {
                let Some(function) = fn_all.get(overload_id) else { continue };
                let name = &function.name;
                let rval = types
                    .get(function.signature.rval)
                    .map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("rval of overload `{name}`")))?;

                let mut args: Vec<HashMap<&str, &str>> = Vec::new();
                for arg in &function.signature.arguments {
                    let arg_ty = types
                        .get(arg.ty)
                        .map(|t| &t.name)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("arg `{}` of overload `{}`", arg.name, name)))?;
                    let mut m = HashMap::new();
                    m.insert("name", arg.name.as_str());
                    m.insert("ty", arg_ty.as_str());
                    args.push(m);
                }

                let mut context = Context::new();

                context.insert("name", name);
                context.insert("symbol", name);
                context.insert("args", &args);
                context.insert("rval", rval);

                let import = templates.render("fns/overload/simple.cs", &context)?;
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
