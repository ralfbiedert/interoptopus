//! Writes function import declarations.

use crate::output::{Output, OutputKind};
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
        output_master: &output::master::Pass,
        fn_maps: &model::fns::originals::Pass,
        types: &model::types::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut imports = Vec::new();

            for (_id, function) in fn_maps.iter() {
                let name = &function.name;
                let rval = types
                    .get(function.signature.rval)
                    .map(|t| &t.name)
                    .ok_or_else(|| crate::Error::MissingTypeName(format!("rval of function `{name}`")))?;

                let mut args: Vec<HashMap<&str, &str>> = Vec::new();
                for arg in &function.signature.arguments {
                    let arg_ty = types
                        .get(arg.ty)
                        .map(|t| &t.name)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("arg `{}` of function `{}`", arg.name, name)))?;
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

                let import = templates.render("fns/rust.cs", &context)?;
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
