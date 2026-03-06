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
        Self { info: PassInfo { name: file!() }, enum_body: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        blittable: &model::types::info::deleteme_blittable::Pass,
        enum_body_unmanaged_variant: &output::types::enums::body_unmanaged_variant::Pass,
        enum_body_unmanaged: &output::types::enums::body_unmanaged::Pass,
        enum_body_to_unmanaged: &output::types::enums::body_to_unmanaged::Pass,
        enum_body_as_unmanaged: &output::types::enums::body_as_unmanaged::Pass,
        enum_body_ctors: &output::types::enums::body_ctors::Pass,
        enum_body_tostring: &output::types::enums::body_tostring::Pass,
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

            let unmanaged_variants = enum_body_unmanaged_variant.get(*type_id).unwrap_or(&[]);
            let unmanaged = enum_body_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let to_unmanaged = enum_body_to_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let as_unmanaged = enum_body_as_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let ctors = enum_body_ctors.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let to_string = enum_body_tostring.get(*type_id).map(|s| s.as_str()).unwrap_or("");

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("unmanaged_variants", &unmanaged_variants);
            context.insert("unmanaged", &unmanaged);
            context.insert("to_unmanaged", &to_unmanaged);
            context.insert("as_unmanaged", &as_unmanaged);
            context.insert("ctors", &ctors);
            context.insert("to_string", &to_string);

            let rendered = templates.render("types/enums/body.cs", &context)?;
            self.enum_body.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.enum_body.get(&type_id)
    }
}
