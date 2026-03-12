//! Renders composite body definitions using the `body.cs` template.

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    composite_body: HashMap<TypeId, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, composite_body: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        struct_class: &model::types::info::struct_class::Pass,
        disposable: &model::types::info::disposable::Pass,
        composite_body_unmanaged: &output::types::composites::body_unmanaged::Pass,
        composite_body_to_unmanaged: &output::types::composites::body_to_unmanaged::Pass,
        composite_body_as_unmanaged: &output::types::composites::body_as_unmanaged::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let type_kind = &ty.kind;
            match type_kind {
                TypeKind::Composite(_) => {}
                _ => continue,
            }

            let name = &ty.name;
            let ty = *type_id;
            let struct_or_class = if struct_class.is_struct(ty) { "struct" } else { "class" };
            let is_disposable = disposable.is_disposable(*type_id).unwrap_or(false);
            let unmanaged = composite_body_unmanaged.get(*type_id).map_or("", std::string::String::as_str);
            let to_unmanaged = composite_body_to_unmanaged.get(*type_id).map_or("", std::string::String::as_str);
            let as_unmanaged = composite_body_as_unmanaged.get(*type_id).map_or("", std::string::String::as_str);

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("is_disposable", &is_disposable);
            context.insert("unmanaged", &unmanaged);
            context.insert("to_unmanaged", &to_unmanaged);
            context.insert("as_unmanaged", &as_unmanaged);

            let rendered = templates.render("types/composite/body.cs", &context)?;
            self.composite_body.insert(*type_id, rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.composite_body.get(&type_id)
    }
}
