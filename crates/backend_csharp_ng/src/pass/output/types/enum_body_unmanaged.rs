//! Renders the `Unmanaged` struct for each enum using the `enum_body_unmanaged.cs` template.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enum_body_unmanaged: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "output/types/enum/body_unmanaged" }, enum_body_unmanaged: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        managed: &output::conversion::managed::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            let data_enum = match type_kind {
                TypeKind::DataEnum(e) => e,
                _ => continue,
            };

            let name = names.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;

            let variants: Vec<HashMap<&str, String>> = data_enum
                .variants
                .iter()
                .filter_map(|v| {
                    let variant_ty = v.ty?;
                    let to_managed = managed.to_managed_suffix(variant_ty).to_string();

                    let mut m = HashMap::new();
                    m.insert("name", v.name.clone());
                    m.insert("id", v.tag.to_string());
                    m.insert("to_managed", to_managed);
                    Some(m)
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("variants", &variants);

            let rendered = templates.render("types/enum/body_unmanaged.cs", &context)?;
            self.enum_body_unmanaged.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.enum_body_unmanaged.get(&type_id)
    }
}
