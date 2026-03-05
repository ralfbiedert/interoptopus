//! Renders enum type definitions using the `enum_ty.cs` template.

use crate::lang::types::{Ownership, TypeKind};
use crate::model::TypeId;
use crate::output::{Output, OutputKind};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enum_ty: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "output_enum_ty" }, enum_ty: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        blittable: &model::types::blittable::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            let data_enum = match type_kind {
                TypeKind::DataEnum(e) => e,
                _ => continue,
            };

            let name = names.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;

            let struct_or_class = match blittable.blittable(*type_id) {
                Some(Ownership::Blittable) => "struct",
                _ => "class",
            };

            let variants: Vec<HashMap<&str, &str>> = data_enum
                .variants
                .iter()
                .filter_map(|v| {
                    let ty = v.ty?;
                    let ty_name = names.name(ty)?;
                    let mut m = HashMap::new();
                    m.insert("name", v.name.as_str());
                    m.insert("type", ty_name.as_str());
                    Some(m)
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("variants", &variants);

            let rendered = templates.render("types/enum_ty.cs", &context)?;
            self.enum_ty.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.enum_ty.get(&type_id)
    }
}
