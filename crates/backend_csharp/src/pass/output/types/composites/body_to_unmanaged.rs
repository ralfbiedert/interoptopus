//! Renders the `ToUnmanaged`/`IntoUnmanaged` method for each composite using the `body_to_unmanaged.cs` template.

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_to_unmanaged: HashMap<TypeId, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_to_unmanaged: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        managed: &output::conversion::unmanaged_conversion::Pass,
        field_conversions: &output::conversion::fields::Pass,
        nullable: &model::types::info::nullable::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let TypeKind::Composite(composite) = &ty.kind else { continue };

            let name = &ty.name;
            let to_unmanaged = managed.to_unmanaged_name(*type_id);

            let fields: Vec<HashMap<&str, String>> = composite
                .fields
                .iter()
                .map(|f| {
                    let suffix = managed.to_unmanaged_suffix(f.ty);
                    let is_nullable = nullable.is_nullable(f.ty).unwrap_or(false);
                    let to_unmanaged = if is_nullable && !suffix.is_empty() {
                        format!("?{suffix} ?? default")
                    } else {
                        suffix.to_string()
                    };

                    let mut m = HashMap::new();
                    m.insert("name", f.name.clone());
                    m.insert("to_unmanaged", to_unmanaged);
                    if let Some(custom) = field_conversions.custom_to_unmanaged(*type_id, &f.name) {
                        m.insert("custom_to_unmanaged", custom.to_string());
                    }
                    m
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("to_unmanaged", to_unmanaged);
            context.insert("fields", &fields);

            let rendered = templates.render("types/composite/body_to_unmanaged.cs", &context)?;
            self.body_to_unmanaged.insert(*type_id, rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_to_unmanaged.get(&type_id)
    }
}
