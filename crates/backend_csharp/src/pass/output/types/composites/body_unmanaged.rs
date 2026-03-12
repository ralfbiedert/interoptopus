//! Renders the `Unmanaged` struct for each composite using the `body_unmanaged.cs` template.

use crate::lang::types::kind::TypeKind;
use crate::lang::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    composite_body_unmanaged: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, composite_body_unmanaged: Default::default() }
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

            let name = &ty.name;

            let fields: Vec<HashMap<&str, String>> = composite
                .fields
                .iter()
                .map(|f| {
                    let ty_name = types.get(f.ty).map(|t| t.name.clone()).unwrap_or_default();
                    let to_managed = managed.to_managed_suffix(f.ty).to_string();

                    let mut m = HashMap::new();
                    m.insert("name", f.name.clone());
                    m.insert("type", ty_name);
                    m.insert("to_managed", to_managed);
                    if let Some(custom) = field_conversions.custom_to_managed(*type_id, &f.name) {
                        m.insert("custom_to_managed", custom.to_string());
                    }
                    m
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("fields", &fields);

            let rendered = templates.render("types/composite/body_unmanaged.cs", &context)?;
            self.composite_body_unmanaged.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.composite_body_unmanaged.get(&type_id)
    }
}
