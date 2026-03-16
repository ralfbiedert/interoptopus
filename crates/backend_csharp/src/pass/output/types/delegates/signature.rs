//! Renders bare function pointer delegates through the `signature.cs` template, grouped per output file.
//!
//! These are simple `[UnmanagedFunctionPointer(CallingConvention.Cdecl)]` delegate declarations
//! for Rust `extern "C" fn(...)` types, as opposed to the full wrapper classes produced by
//! the `class` pass for named callbacks.

use crate::lang::types::kind::{DelegateKind, TypeKind};
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    delegates: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, delegates: HashMap::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass, types: &model::types::all::Pass) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered = Vec::new();

            for (type_id, ty) in types.iter() {
                if !output_master.type_belongs_to(*type_id, file) {
                    continue;
                }

                let delegate = match &ty.kind {
                    TypeKind::Delegate(d) if d.kind == DelegateKind::Signature => d,
                    _ => continue,
                };
                let signature = &delegate.signature;
                let name = &ty.name;

                let rval_managed = types.get(signature.rval).map_or_else(|| "void".to_string(), |t| t.name.clone());

                let mut args: Vec<HashMap<String, String>> = Vec::new();
                for arg in &signature.arguments {
                    let Some(arg_managed) = types.get(arg.ty).map(|t| &t.name) else {
                        continue;
                    };
                    let mut m = HashMap::new();
                    m.insert("name".to_string(), arg.name.clone());
                    m.insert("managed_type".to_string(), arg_managed.clone());
                    args.push(m);
                }

                let mut context = Context::new();
                context.insert("name", name);
                context.insert("rval_managed", &rval_managed);
                context.insert("args", &args);

                let r = templates.render("types/delegate/signature.cs", &context)?;
                rendered.push(r);
            }

            rendered.sort();
            self.delegates.insert(file.clone(), rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn delegates_for(&self, output: &Output) -> Option<&[String]> {
        self.delegates.get(output).map(std::vec::Vec::as_slice)
    }
}
