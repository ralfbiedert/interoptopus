//! Renders per-variant unmanaged struct definitions using the `enum_body_unmanaged.cs` template.

use crate::lang::TypeId;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enum_body_unmanaged: HashMap<TypeId, Vec<String>>,
}

impl Pass {
    #[must_use] 
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, enum_body_unmanaged: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        unmanaged_names: &output::conversion::unmanaged_names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let type_kind = &ty.kind;
            let data_enum = match type_kind {
                TypeKind::DataEnum(e) => e,
                TypeKind::TypePattern(TypePattern::Result(_, _, e)) => e,
                TypeKind::TypePattern(TypePattern::Option(_, e)) => e,
                _ => continue,
            };

            let mut rendered_variants = Vec::new();

            for variant in &data_enum.variants {
                let Some(variant_ty) = variant.ty else {
                    continue;
                };

                let Some(variant_type) = unmanaged_names.name(variant_ty) else {
                    continue;
                };

                let mut context = Context::new();
                context.insert("variant", &variant.name);
                context.insert("unmanaged_name", variant_type);

                let rendered = templates.render("types/enums/body_unmanaged_variant.cs", &context)?;
                rendered_variants.push(rendered);
            }

            self.enum_body_unmanaged.insert(*type_id, rendered_variants);
        }

        Ok(())
    }

    #[must_use] 
    pub fn get(&self, type_id: TypeId) -> Option<&[String]> {
        self.enum_body_unmanaged.get(&type_id).map(std::vec::Vec::as_slice)
    }
}
