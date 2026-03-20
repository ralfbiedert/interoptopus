//! Renders `Vec<T>` pattern types per output file.
//!
//! For each Vec type, determines whether to use the "fast" (blittable) or
//! "marshalling" template based on the element type's `ManagedConversion`.
//! Elements with `AsIs` or `To` conversion are blittable; elements with `Into`
//! conversion require per-element marshalling. The nested `InteropHelper` class
//! embeds the `vec_create` / `vec_destroy` entry points discovered by the
//! `model::rust::pattern::vec` pass.

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
    vecs: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, vecs: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        managed_conversion: &model::rust::types::info::managed_conversion::Pass,
        unmanaged_names: &output::rust::conversion::unmanaged_names::Pass,
        pattern_vec: &model::rust::pattern::vec::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered_vecs = Vec::new();

            for (type_id, ty) in types.iter() {
                if !output_master.type_belongs_to(*type_id, file) {
                    continue;
                }

                let element_ty_id = match &ty.kind {
                    TypeKind::TypePattern(TypePattern::Vec(t)) => *t,
                    _ => continue,
                };

                let Some(helpers) = pattern_vec.helpers(*type_id) else { continue };
                let Some(element_ty) = types.get(element_ty_id) else { continue };
                let element_name = &element_ty.name;

                let is_blittable = matches!(managed_conversion.managed_conversion(element_ty_id), Some(ManagedConversion::AsIs | ManagedConversion::To));

                let rendered = if is_blittable {
                    let mut context = Context::new();
                    context.insert("name", &ty.name);
                    context.insert("element_type", element_name);
                    context.insert("create_entry_point", &helpers.create_entry_point);
                    context.insert("destroy_entry_point", &helpers.destroy_entry_point);
                    templates.render("rust/pattern/vec/fast.cs", &context)?
                } else {
                    let unmanaged_name = unmanaged_names.name(element_ty_id).cloned().unwrap_or_else(|| format!("{element_name}.Unmanaged"));

                    let mut context = Context::new();
                    context.insert("name", &ty.name);
                    context.insert("element_type", element_name);
                    context.insert("unmanaged_element_type", &unmanaged_name);
                    context.insert("create_entry_point", &helpers.create_entry_point);
                    context.insert("destroy_entry_point", &helpers.destroy_entry_point);
                    templates.render("rust/pattern/vec/marshalling.cs", &context)?
                };

                rendered_vecs.push(rendered);
            }

            rendered_vecs.sort();

            self.vecs.insert(file.clone(), rendered_vecs);
        }

        Ok(())
    }

    #[must_use]
    pub fn vecs_for(&self, output: &Output) -> Option<&[String]> {
        self.vecs.get(output).map(std::vec::Vec::as_slice)
    }
}
