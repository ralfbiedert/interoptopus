//! Renders `ExceptionForVariant()` method for each enum using the
//! `body_exception_for_variant.cs` template.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_exception_for_variant: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_exception_for_variant: Default::default() }
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
            context.insert("variants", &variants);

            let rendered = templates.render("types/enums/body_exception_for_variant.cs", &context)?;
            self.body_exception_for_variant.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_exception_for_variant.get(&type_id)
    }
}
