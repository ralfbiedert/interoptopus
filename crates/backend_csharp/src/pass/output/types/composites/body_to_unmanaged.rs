//! Renders the `ToUnmanaged`/`IntoUnmanaged` method for each composite using the `body_to_unmanaged.cs` template.

use crate::lang::types::TypeKind;
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
        field_conversions: &output::conversion::fields::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let type_kind = &ty.kind;
            let composite = match type_kind {
                TypeKind::Composite(c) => c,
                _ => continue,
            };

            let name = types.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;
            let to_unmanaged = managed.to_unmanaged_name(*type_id);

            let fields: Vec<HashMap<&str, String>> = composite
                .fields
                .iter()
                .map(|f| {
                    let to_unmanaged = managed.to_unmanaged_suffix(f.ty).to_string();

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

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_to_unmanaged.get(&type_id)
    }
}
