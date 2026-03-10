//! Renders constructors, variant checks, and conversion methods for each enum
//! using the `body_ctors.cs` template.

use crate::lang::types::{TypeKind, TypePattern};
use crate::lang::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::{Context, Value};
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
                TypeKind::TypePattern(TypePattern::Result(_, _, e)) => e,
                TypeKind::TypePattern(TypePattern::Option(_, e)) => e,
                _ => continue,
            };

            let name = names.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;

            let variants: Vec<HashMap<&str, Value>> = data_enum
                .variants
                .iter()
                .map(|v| {
                    let has_payload = v.ty.is_some();
                    let type_name = v.ty.and_then(|ty| names.name(ty)).cloned().unwrap_or_default();

                    let mut m = HashMap::new();
                    m.insert("name", Value::String(v.name.clone()));
                    m.insert("id", Value::Number(v.tag.into()));
                    m.insert("has_payload", Value::Bool(has_payload));
                    m.insert("type", Value::String(type_name));
                    m
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("variants", &variants);

            let rendered = templates.render("types/enums/body_ctors.cs", &context)?;
            self.body_ctors.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_ctors.get(&type_id)
    }
}
