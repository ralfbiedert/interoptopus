//! Writes function import declarations.

use crate::output::{Output, OutputKind};
use crate::pass::{model_fn_map, model_type_names, output_master, OutputResult, PassInfo};
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

    pub fn process(
        &mut self,
        _pass_meta: &mut super::PassMeta,
        output_master: &output_master::Pass,
        fn_maps: &model_fn_map::Pass,
        type_names: &model_type_names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for output in output_master.outputs_of(OutputKind::Csharp) {
            let mut imports = Vec::new();

            for (_id, function) in fn_maps.iter() {
                let name = &function.name;
                for overload in &function.overloads {
                    let rval = type_names
                        .name(overload.signature.rval)
                        .ok_or_else(|| crate::Error::MissingTypeName(format!("rval of function `{}`", name)))?;

                    let mut args: Vec<HashMap<&str, &str>> = Vec::new();
                    for arg in &overload.signature.arguments {
                        let arg_ty = type_names
                            .name(arg.ty)
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
