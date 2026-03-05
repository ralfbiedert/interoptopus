//! Renders enum body definitions using the `enum_body.cs` template.

use crate::lang::types::{Ownership, TypeKind};
use crate::model::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enum_body: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "output_enum_body" }, enum_body: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        blittable: &model::types::blittable::Pass,
        enum_body_unmanaged: &output::types::enum_body_unmanaged_variant::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            match type_kind {
                TypeKind::DataEnum(_) => {}
                _ => continue,
            }

            let name = names.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;

            let struct_or_class = match blittable.blittable(*type_id) {
                Some(Ownership::Blittable) => "struct",
                _ => "class",
            };

            let unmanaged = enum_body_unmanaged.get(*type_id).unwrap_or(&[]);

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("unmanaged_variants", &unmanaged);

            let rendered = templates.render("types/enum_body.cs", &context)?;
            self.enum_body.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.enum_body.get(&type_id)
    }
}
