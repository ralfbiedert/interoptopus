//! Writes function import declarations.

use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, format_docs, model, output};
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

            for (&id, function) in fns_all.originals() {
                if !output_master.fn_belongs_to(id, output) {
                    continue;
                }

                let name = &function.name;
                let rval_type = types
                    .get(function.signature.rval)
                    .ok_or_else(|| crate::Error::from(format!("rval of function `{name}`")))?;
                let rval = &rval_type.name;

                let mut args: Vec<HashMap<&str, String>> = Vec::new();
                for arg in &function.signature.arguments {
                    let arg_type = types
                        .get(arg.ty)
                        .ok_or_else(|| crate::Error::from(format!("arg `{}` of function `{}`", arg.name, name)))?;
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

                let rval_decorator = rval_type.decorators.rval.as_ref().map(|d| match d {
                    crate::lang::types::RvalDecorator::MarshalAs(m) => format!("return: MarshalAs({m})"),
                    crate::lang::types::RvalDecorator::MarshalUsing(t) => format!("return: MarshalUsing(typeof({t}))"),
                });

                let docs = format_docs(&function.docs);

                context.insert("name", name);
                context.insert("symbol", name);
                context.insert("args", &args);
                context.insert("rval", rval);
                context.insert("rval_decorator", &rval_decorator);
                context.insert("docs", &docs);
                context.insert("visibility", &function.visibility.to_string());

                let import = templates.render("rust/fns/rust.cs", &context)?;
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
