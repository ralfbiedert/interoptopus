//! Renders slice types (`Slice<T>` and `SliceMut<T>`) per output file.
//!
//! For each slice type, determines whether to use the "fast" (representation-
//! identical) or "marshalling" template based on the element type's
//! `ManagedConversion`. Only `AsIs` elements can be projected directly over
//! native memory. `To` elements require per-element conversion, while `Into`
//! elements cannot be safely read from borrowed memory.

use crate::lang::TypeId;
use crate::lang::types::ManagedConversion;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    slices: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, slices: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        managed_conversion: &model::common::types::info::managed_conversion::Pass,
        unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered_slices = Vec::new();

            for (type_id, ty) in types.iter() {
                if !output_master.type_belongs_to(*type_id, file) {
                    continue;
                }

                let (element_ty_id, is_mut) = match &ty.kind {
                    TypeKind::TypePattern(TypePattern::Slice(t)) => (*t, false),
                    TypeKind::TypePattern(TypePattern::SliceMut(t)) => (*t, true),
                    _ => continue,
                };

                let Some(element_ty) = types.get(element_ty_id) else { continue };
                let element_name = &element_ty.name;

                let Some(element_conversion) = managed_conversion.managed_conversion(element_ty_id) else {
                    continue;
                };

                let method = if is_mut { "SliceMut" } else { "Slice" };

                let rendered = if element_conversion == ManagedConversion::AsIs {
                    let mut context = Context::new();
                    context.insert("name", &ty.name);
                    context.insert("element_type", element_name);
                    context.insert("is_mut", &is_mut);
                    context.insert("method", method);
                    templates.render("rust/pattern/slice/fast.cs", &context)?
                } else {
                    let unmanaged_name = unmanaged_names.name(element_ty_id).cloned().unwrap_or_else(|| format!("{element_name}.Unmanaged"));

                    let mut context = Context::new();
                    context.insert("name", &ty.name);
                    context.insert("element_type", element_name);
                    context.insert("unmanaged_element_type", &unmanaged_name);
                    context.insert("method", method);
                    let element_to_managed = match element_conversion {
                        ManagedConversion::Into => "IntoManaged",
                        _ => "ToManaged",
                    };
                    context.insert("element_to_managed", element_to_managed);
                    context.insert("has_indexer", &(element_conversion == ManagedConversion::To));
                    templates.render("rust/pattern/slice/marshalling.cs", &context)?
                };

                rendered_slices.push(rendered);
            }

            rendered_slices.sort();

            self.slices.insert(file.clone(), rendered_slices);
        }

        Ok(())
    }

    #[must_use]
    pub fn slices_for(&self, output: &Output) -> Option<&[String]> {
        self.slices.get(output).map(std::vec::Vec::as_slice)
    }
}
