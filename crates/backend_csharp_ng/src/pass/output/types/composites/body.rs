//! Renders composite body definitions using the `body.cs` template.

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    composite_body: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, composite_body: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        struct_class: &model::types::info::struct_class::Pass,
        disposable: &model::types::info::disposable::Pass,
        composite_body_unmanaged: &output::types::composites::body_unmanaged::Pass,
        composite_body_to_unmanaged: &output::types::composites::body_to_unmanaged::Pass,
        composite_body_as_unmanaged: &output::types::composites::body_as_unmanaged::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            match type_kind {
                TypeKind::Composite(_) => {}
                _ => continue,
            }

            let name = names.name(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;
            let ty = *type_id;
            let struct_or_class = if struct_class.is_struct(ty) { "struct" } else { "class" };
            let is_disposable = disposable.is_disposable(*type_id).unwrap_or(false);
            let unmanaged = composite_body_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let to_unmanaged = composite_body_to_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let as_unmanaged = composite_body_as_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");

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

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.composite_body.get(&type_id)
    }
}
