//! Renders the `ToUnmanaged`/`IntoUnmanaged` method for each enum using the `body_to_unmanaged.cs` template.

use crate::lang::types::{TypeKind, TypePattern};
use crate::lang::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_to_unmanaged: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_to_unmanaged: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        managed: &output::conversion::unmanaged_conversion::Pass,
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

            let name = types.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;
            let to_unmanaged = managed.to_unmanaged_name(*type_id);

            let variants: Vec<HashMap<&str, String>> = data_enum
                .variants
                .iter()
                .filter_map(|v| {
                    let variant_ty = v.ty?;
                    let to_unmanaged = managed.to_unmanaged_suffix(variant_ty).to_string();

                    let mut m = HashMap::new();
                    m.insert("name", v.name.clone());
                    m.insert("id", v.tag.to_string());
                    m.insert("to_unmanaged", to_unmanaged);
                    Some(m)
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("to_unmanaged", to_unmanaged);
            context.insert("variants", &variants);

            let rendered = templates.render("types/enums/body_to_unmanaged.cs", &context)?;
            self.body_to_unmanaged.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_to_unmanaged.get(&type_id)
    }
}
