//! Renders the `ToUnmanaged`/`IntoUnmanaged` method for each composite using the `body_to_unmanaged.cs` template.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
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
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        managed: &output::conversion::managed::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            let composite = match type_kind {
                TypeKind::Composite(c) => c,
                _ => continue,
            };

            let name = names.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;
            let to_unmanaged = managed.to_unmanaged_name(*type_id);

            let fields: Vec<HashMap<&str, String>> = composite
                .fields
                .iter()
                .map(|f| {
                    let to_unmanaged = managed.to_unmanaged_suffix(f.ty).to_string();

                    let custom_to_unmanaged = render_custom_to_unmanaged(templates, kinds, names, &f.name, f.ty);

                    let mut m = HashMap::new();
                    m.insert("name", f.name.clone());
                    m.insert("to_unmanaged", to_unmanaged);
                    if let Some(custom) = custom_to_unmanaged {
                        m.insert("custom_to_unmanaged", custom);
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

fn render_custom_to_unmanaged(
    templates: &interoptopus_backends::template::TemplateEngine,
    kinds: &model::types::kind::Pass,
    names: &model::types::names::Pass,
    field_name: &str,
    field_ty: TypeId,
) -> Option<String> {
    match kinds.get(field_ty)? {
        TypeKind::Array(a) => {
            let element_type = names.name(a.ty)?;
            let mut ctx = Context::new();
            ctx.insert("field", field_name);
            ctx.insert("element_type", element_type.as_str());
            ctx.insert("len", &a.len);
            templates.render("types/composite/fields/array_to_unmanaged.cs", &ctx).ok()
        }
        _ => None,
    }
}
