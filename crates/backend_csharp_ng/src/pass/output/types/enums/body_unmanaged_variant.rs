//! Renders per-variant unmanaged struct definitions using the `enum_body_unmanaged.cs` template.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enum_body_unmanaged: HashMap<TypeId, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, enum_body_unmanaged: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        struct_class: &model::types::info::struct_class::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            let data_enum = match type_kind {
                TypeKind::DataEnum(e) => e,
                _ => continue,
            };

            let mut rendered_variants = Vec::new();

            for variant in &data_enum.variants {
                let Some(variant_ty) = variant.ty else {
                    continue;
                };

                let Some(variant_type_name) = names.name(variant_ty) else {
                    continue;
                };

                let variant_type = if struct_class.is_struct(variant_ty) {
                    variant_type_name.to_string()
                } else {
                    format!("{variant_type_name}.Unmanaged")
                };

                let mut context = Context::new();
                context.insert("variant", &variant.name);
                context.insert("variant_type", &variant_type);

                let rendered = templates.render("types/enums/body_unmanaged_variant.cs", &context)?;
                rendered_variants.push(rendered);
            }

            self.enum_body_unmanaged.insert(*type_id, rendered_variants);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&[String]> {
        self.enum_body_unmanaged.get(&type_id).map(|v| v.as_slice())
    }
}
