//! Renders the `ToString` override for each enum using the `body_tostring.cs` template.

use crate::lang::types::{TypeKind, TypePattern};
use crate::lang::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::{Context, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_tostring: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_tostring: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass, kinds: &model::types::kind::Pass) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            let data_enum = match type_kind {
                TypeKind::DataEnum(e) => e,
                TypeKind::TypePattern(TypePattern::Result(_, _, e)) => e,
                TypeKind::TypePattern(TypePattern::Option(_, e)) => e,
                _ => continue,
            };

            let variants: Vec<HashMap<&str, Value>> = data_enum
                .variants
                .iter()
                .map(|v| {
                    let mut m = HashMap::new();
                    m.insert("name", Value::String(v.name.clone()));
                    m.insert("id", Value::Number(v.tag.into()));
                    m.insert("has_payload", Value::Bool(v.ty.is_some()));
                    m
                })
                .collect();

            let mut context = Context::new();
            context.insert("variants", &variants);

            let rendered = templates.render("types/enums/body_tostring.cs", &context)?;
            self.body_tostring.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_tostring.get(&type_id)
    }
}
