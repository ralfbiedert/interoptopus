//! Renders the `AsUnmanaged` method for each composite using the `body_as_unmanaged.cs` template.

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_as_unmanaged: HashMap<TypeId, String>,
}

impl Pass {
    #[must_use] 
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_as_unmanaged: HashMap::default() }
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
            let TypeKind::Composite(composite) = &ty.kind else { continue };

            let name = &ty.name;

            let fields: Vec<HashMap<&str, String>> = composite
                .fields
                .iter()
                .map(|f| {
                    let as_unmanaged = managed.as_unmanaged_suffix(f.ty).to_string();

                    let mut m = HashMap::new();
                    m.insert("name", f.name.clone());
                    m.insert("as_unmanaged", as_unmanaged);
                    if let Some(custom) = field_conversions.custom_to_unmanaged(*type_id, &f.name) {
                        m.insert("custom_to_unmanaged", custom.to_string());
                    }
                    m
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("fields", &fields);

            let rendered = templates.render("types/composite/body_as_unmanaged.cs", &context)?;
            self.body_as_unmanaged.insert(*type_id, rendered);
        }

        Ok(())
    }

    #[must_use] 
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_as_unmanaged.get(&type_id)
    }
}
