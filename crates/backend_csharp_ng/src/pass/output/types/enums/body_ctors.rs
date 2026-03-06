//! Renders constructors, variant checks, and conversion methods for each enum
//! using the `body_ctors.cs` template.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_ctors: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_ctors: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
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
                .map(|v| {
                    let has_payload = v.ty.is_some();
                    let type_name = v.ty.and_then(|ty| names.name(ty)).cloned().unwrap_or_default();

                    let mut m = HashMap::new();
                    m.insert("name", v.name.clone());
                    m.insert("id", v.tag.to_string());
                    m.insert("has_payload", has_payload.to_string());
                    m.insert("type", type_name);
                    m
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("variants", &variants);

            let rendered = templates.render("types/enum/body_ctors.cs", &context)?;
            self.body_ctors.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_ctors.get(&type_id)
    }
}
