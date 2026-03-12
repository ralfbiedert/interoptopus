//! Renders async trampoline classes and their static field declarations.
//!
//! For each unique async Result type used by async callbacks, generates:
//! - A trampoline class (`AsyncTrampoline*`) that manages in-flight tasks
//! - A static field declaration for the `Interop` class

use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::lang::types::ManagedConversion;
use crate::output::{Output, OutputKind};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    trampolines: HashMap<Output, Vec<String>>,
    trampoline_fields: HashMap<Output, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: file!() },
            trampolines: Default::default(),
            trampoline_fields: Default::default(),
        }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        overload_body: &model::fns::overload::body::Pass,
        types: &model::types::all::Pass,
        managed_conversion: &model::types::info::managed_conversion::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut rendered_trampolines = Vec::new();
            let mut rendered_fields = Vec::new();

            for &result_ty_id in overload_body.trampoline_types() {
                let Some(result_ty) = types.get(result_ty_id) else { continue };
                let result_ty_name = &result_ty.name;

                // The trampoline class name and field name
                let trampoline_name = format!("AsyncTrampoline{}", result_ty_name);
                let trampoline_field = format!("_trampoline{}", result_ty_name);

                // Determine the Task<T> inner type from the Result's Ok variant
                let (task_inner_ty, is_task_void) = match &result_ty.kind {
                    TypeKind::TypePattern(TypePattern::Result(ok_ty, _, _)) => {
                        let ok_kind = types.get(*ok_ty).map(|t| &t.kind);
                        if matches!(ok_kind, Some(TypeKind::Primitive(Primitive::Void))) {
                            ("void".to_string(), true)
                        } else {
                            let ok_name = types.get(*ok_ty).map(|t| t.name.clone()).unwrap_or_else(|| "void".to_string());
                            (ok_name, false)
                        }
                    }
                    _ => continue,
                };

                // Check if the result type has an unmanaged representation
                let has_unmanaged = matches!(
                    managed_conversion.managed_conversion(result_ty_id),
                    Some(ManagedConversion::To | ManagedConversion::Into)
                );

                let unmanaged_result_ty = if has_unmanaged {
                    format!("{}.Unmanaged", result_ty_name)
                } else {
                    result_ty_name.clone()
                };

                let mut context = Context::new();
                context.insert("trampoline_name", &trampoline_name);
                context.insert("result_ty_name", result_ty_name);
                context.insert("unmanaged_result_ty", &unmanaged_result_ty);
                context.insert("task_inner_ty", &task_inner_ty);
                context.insert("is_task_void", &is_task_void);
                context.insert("has_unmanaged", &has_unmanaged);

                let rendered = templates.render("types/asynk/trampoline.cs", &context)?;
                rendered_trampolines.push(rendered);

                // Render the static field declaration
                let mut field_context = Context::new();
                field_context.insert("trampoline_name", &trampoline_name);
                field_context.insert("trampoline_field", &trampoline_field);
                let field_rendered = templates.render("types/asynk/trampoline_field.cs", &field_context)?;
                rendered_fields.push(field_rendered);
            }

            rendered_trampolines.sort();
            rendered_fields.sort();

            self.trampolines.insert(file.clone(), rendered_trampolines);
            self.trampoline_fields.insert(file.clone(), rendered_fields);
        }

        Ok(())
    }

    pub fn trampolines_for(&self, output: &Output) -> Option<&[String]> {
        self.trampolines.get(output).map(|s| s.as_slice())
    }

    pub fn trampoline_fields_for(&self, output: &Output) -> Option<&[String]> {
        self.trampoline_fields.get(output).map(|s| s.as_slice())
    }
}
