//! Renders delegate type definitions through the `all.cs` template, grouped per output file.

use crate::lang::types::{Primitive, TypeKind, TypePattern};
use crate::lang::TypeId;
use crate::output::{Output, OutputKind};
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
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, delegates: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        unmanaged_names: &output::conversion::unmanaged_names::Pass,
        unmanaged_conversion: &output::conversion::unmanaged_conversion::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut rendered_delegates = Vec::new();

            for (type_id, type_kind) in kinds.iter() {
                let delegate = match type_kind {
                    TypeKind::Delegate(d) => d,
                    TypeKind::TypePattern(TypePattern::NamedCallback(d)) => d,
                    _ => continue,
                };
                let signature = &delegate.signature;

                let Some(name) = names.name(*type_id) else {
                    continue;
                };

                // Determine return type info
                let rval_kind = kinds.get(signature.rval);
                let rval_managed = names.name(signature.rval).cloned().unwrap_or_else(|| "void".to_string());
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
                    let Some(arg_managed) = names.name(arg.ty) else {
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

    pub fn delegates_for(&self, output: &Output) -> Option<&[String]> {
        self.delegates.get(output).map(|s| s.as_slice())
    }
}
