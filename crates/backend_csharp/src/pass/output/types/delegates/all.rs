//! Renders delegate type definitions through the `all.cs` template, grouped per output file.

use crate::lang::types::kind::{DelegateKind, Primitive, TypeKind};
use crate::lang::TypeId;
use crate::output::{FileType, Output};
use crate::pass::{model, output, OutputResult, PassInfo};
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

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        unmanaged_names: &output::conversion::unmanaged_names::Pass,
        unmanaged_conversion: &output::conversion::unmanaged_conversion::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered_delegates = Vec::new();

            for (type_id, ty) in types.iter() {
                if !output_master.type_belongs_to(*type_id, file) {
                    continue;
                }

                let type_kind = &ty.kind;
                let delegate = match type_kind {
                    TypeKind::Delegate(d) if d.kind == DelegateKind::Class => d,
                    _ => continue,
                };
                let signature = &delegate.signature;

                let name = &ty.name;

                // Determine return type info
                let rval_kind = types.get(signature.rval).map(|t| &t.kind);
                let rval_managed = types.get(signature.rval).map_or_else(|| "void".to_string(), |t| t.name.clone());
                let is_void = matches!(rval_kind, Some(TypeKind::Primitive(Primitive::Void)));

                let rval_unmanaged = if is_void {
                    "void".to_string()
                } else {
                    unmanaged_names.name(signature.rval).cloned().unwrap_or_else(|| rval_managed.clone())
                };

                let rval_to_unmanaged = if is_void {
                    String::new()
                } else {
                    unmanaged_conversion.to_unmanaged_suffix(signature.rval).to_string()
                };

                let rval_to_managed = if is_void {
                    String::new()
                } else {
                    unmanaged_conversion.to_managed_suffix(signature.rval).to_string()
                };

                // Build argument list (excluding callback_data which is always appended in the template)
                let mut args: Vec<HashMap<String, String>> = Vec::new();
                for arg in &signature.arguments {
                    let Some(arg_managed) = types.get(arg.ty).map(|t| &t.name) else {
                        continue;
                    };

                    let arg_unmanaged = unmanaged_names.name(arg.ty).cloned().unwrap_or_else(|| arg_managed.clone());
                    let to_managed = unmanaged_conversion.to_managed_suffix(arg.ty).to_string();
                    let to_unmanaged = unmanaged_conversion.to_unmanaged_suffix(arg.ty).to_string();

                    let mut m = HashMap::new();
                    m.insert("name".to_string(), arg.name.clone());
                    m.insert("managed_type".to_string(), arg_managed.clone());
                    m.insert("unmanaged_name".to_string(), arg_unmanaged);
                    m.insert("to_managed".to_string(), to_managed);
                    m.insert("to_unmanaged".to_string(), to_unmanaged);
                    args.push(m);
                }

                let mut context = Context::new();
                context.insert("name", name);
                context.insert("is_void", &is_void);
                context.insert("rval_managed", &rval_managed);
                context.insert("rval_unmanaged_name", &rval_unmanaged);
                context.insert("rval_to_unmanaged", &rval_to_unmanaged);
                context.insert("rval_to_managed", &rval_to_managed);
                context.insert("args", &args);

                let rendered = templates.render("types/delegate/all.cs", &context)?;
                rendered_delegates.push(rendered);
            }

            rendered_delegates.sort();

            self.delegates.insert(file.clone(), rendered_delegates);
        }

        Ok(())
    }

    #[must_use]
    pub fn delegates_for(&self, output: &Output) -> Option<&[String]> {
        self.delegates.get(output).map(std::vec::Vec::as_slice)
    }
}
