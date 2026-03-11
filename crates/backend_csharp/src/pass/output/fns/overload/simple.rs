//! Writes function import declarations for simple overloads.
//!
//! Simple overloads do not contain a body — they are plain DllImport declarations.
//! The overload IDs come from the simple model pass, with the actual Function
//! objects looked up from `fns::all`.

use crate::output::{Output, OutputKind};
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
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, fn_imports: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        overload_simple: &model::fns::overload::simple::Pass,
        fn_all: &model::fns::all::Pass,
        type_names: &model::types::names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut imports = Vec::new();

            for overload_id in overload_simple.iter_overloads() {
                let Some(function) = fn_all.get(overload_id) else { continue };
                let name = &function.name;
                let rval = type_names
                    .name(function.signature.rval)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("rval of overload `{}`", name)))?;

                let mut args: Vec<HashMap<&str, &str>> = Vec::new();
                for arg in &function.signature.arguments {
                    let arg_ty = type_names
                        .name(arg.ty)
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

    pub fn imports_for(&self, output: &Output) -> Option<&[String]> {
        self.fn_imports.get(output).map(|s| s.as_slice())
    }
}
