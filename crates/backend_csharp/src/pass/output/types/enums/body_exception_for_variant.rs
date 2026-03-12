//! Renders `ExceptionForVariant()` method for each enum using the
//! `body_exception_for_variant.cs` template.

use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::lang::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::{Context, Value};
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
        types: &model::types::all::Pass,
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

            let variants: Vec<HashMap<&str, Value>> = data_enum
                .variants
                .iter()
                .map(|v| {
                    let has_payload = v.ty.is_some();
                    let type_name = v.ty.and_then(|ty| types.get(ty).map(|t| &t.name)).cloned().unwrap_or_default();

                    let mut m = HashMap::new();
                    m.insert("name", Value::String(v.name.clone()));
                    m.insert("id", Value::Number(v.tag.into()));
                    m.insert("has_payload", Value::Bool(has_payload));
                    m.insert("type", Value::String(type_name));
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
