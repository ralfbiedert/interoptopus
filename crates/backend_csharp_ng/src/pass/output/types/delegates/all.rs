//! Renders delegate type definitions through the `all.cs` template, grouped per output file.

use crate::lang::types::{Primitive, TypeKind};
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
        conversion_invoke: &output::conversion::managed::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut rendered_delegates = Vec::new();

            for (type_id, type_kind) in kinds.iter() {
                let signature = match type_kind {
                    TypeKind::Delegate(sig) => sig,
                    _ => continue,
                };

                let Some(name) = names.name(*type_id) else {
                    continue;
                };

                // Determine return type info
                let rval_kind = kinds.get(signature.rval);
                let rval_managed = names.name(signature.rval).cloned().unwrap_or_else(|| "void".to_string());
                let is_void = matches!(rval_kind, Some(TypeKind::Primitive(Primitive::Void)));
                let has_return = !is_void;

                let rval_unmanaged = if is_void {
                    "void".to_string()
                } else {
                    unmanaged_type_name(&rval_managed, conversion_invoke, signature.rval)
                };

                let rval_to_unmanaged = if is_void {
                    String::new()
                } else {
                    conversion_invoke.to_unmanaged_suffix(signature.rval).to_string()
                };

                // Build argument list (excluding callback_data which is always appended in the template)
                let mut args: Vec<HashMap<String, String>> = Vec::new();
                for arg in &signature.arguments {
                    let Some(arg_managed) = names.name(arg.ty) else {
                        continue;
                    };

                    let arg_unmanaged = unmanaged_type_name(arg_managed, conversion_invoke, arg.ty);
                    let to_managed = conversion_invoke.to_managed_suffix(arg.ty).to_string();

                    let mut m = HashMap::new();
                    m.insert("name".to_string(), arg.name.clone());
                    m.insert("managed_type".to_string(), arg_managed.clone());
                    m.insert("unmanaged_type".to_string(), arg_unmanaged);
                    m.insert("to_managed".to_string(), to_managed);
                    args.push(m);
                }

                let mut context = Context::new();
                context.insert("name", name);
                context.insert("has_return", &has_return);
                context.insert("rval_managed", &rval_managed);
                context.insert("rval_unmanaged", &rval_unmanaged);
                context.insert("rval_to_unmanaged", &rval_to_unmanaged);
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

/// Returns the unmanaged type name for a given type.
/// AsIs types use the same name; To/Into types use `Name.Unmanaged`.
fn unmanaged_type_name(managed_name: &str, conversion_invoke: &output::conversion::managed::Pass, ty: crate::model::TypeId) -> String {
    let suffix = conversion_invoke.to_unmanaged_suffix(ty);
    if suffix.is_empty() {
        // AsIs - same type
        managed_name.to_string()
    } else {
        // To or Into - use Unmanaged nested type
        format!("{}.Unmanaged", managed_name)
    }
}
