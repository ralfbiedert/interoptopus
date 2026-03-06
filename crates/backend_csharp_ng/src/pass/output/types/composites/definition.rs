//! Renders composite type definitions using the `definition.cs` template.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    composite_ty: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, composite_ty: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        struct_class: &model::types::info::struct_class::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            let composite = match type_kind {
                TypeKind::Composite(c) => c,
                _ => continue,
            };

            let name = names.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;
            let ty = *type_id;
            let struct_or_class = if struct_class.is_struct(ty) { "struct" } else { "class" };

            let fields: Vec<HashMap<&str, &str>> = composite
                .fields
                .iter()
                .filter_map(|f| {
                    let ty_name = names.name(f.ty)?;
                    let mut m = HashMap::new();
                    m.insert("name", f.name.as_str());
                    m.insert("type", ty_name.as_str());
                    Some(m)
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("fields", &fields);

            let rendered = templates.render("types/composite/definition.cs", &context)?;
            self.composite_ty.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.composite_ty.get(&type_id)
    }
}
