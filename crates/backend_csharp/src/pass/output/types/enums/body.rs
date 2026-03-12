//! Renders enum body definitions using the `enum_body.cs` template.

use crate::lang::types::{TypeKind, TypePattern};
use crate::lang::TypeId;
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
        struct_class: &model::types::info::struct_class::Pass,
        disposable: &model::types::info::disposable::Pass,
        enum_body_unmanaged_variant: &output::types::enums::body_unmanaged_variant::Pass,
        enum_body_unmanaged: &output::types::enums::body_unmanaged::Pass,
        enum_body_to_unmanaged: &output::types::enums::body_to_unmanaged::Pass,
        enum_body_as_unmanaged: &output::types::enums::body_as_unmanaged::Pass,
        enum_body_ctors: &output::types::enums::body_ctors::Pass,
        enum_body_exception_for_variant: &output::types::enums::body_exception_for_variant::Pass,
        enum_body_tostring: &output::types::enums::body_tostring::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, type_kind) in kinds.iter() {
            match type_kind {
                TypeKind::DataEnum(_) => {}
                TypeKind::TypePattern(TypePattern::Result(_, _, _)) => {}
                TypeKind::TypePattern(TypePattern::Option(_, _)) => {}
                _ => continue,
            }

            let name = names.get(*type_id).ok_or_else(|| crate::Error::MissingTypeName(format!("{type_id:?}")))?;

            let ty = *type_id;
            let struct_or_class = if struct_class.is_struct(ty) { "struct" } else { "class" };
            let is_disposable = disposable.is_disposable(*type_id).unwrap_or(false);

            let unmanaged_variants = enum_body_unmanaged_variant.get(*type_id).unwrap_or(&[]);
            let unmanaged = enum_body_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let to_unmanaged = enum_body_to_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let as_unmanaged = enum_body_as_unmanaged.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let ctors = enum_body_ctors.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let exception_for_variant = enum_body_exception_for_variant.get(*type_id).map(|s| s.as_str()).unwrap_or("");
            let to_string = enum_body_tostring.get(*type_id).map(|s| s.as_str()).unwrap_or("");

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("is_disposable", &is_disposable);
            context.insert("unmanaged_variants", &unmanaged_variants);
            context.insert("unmanaged", &unmanaged);
            context.insert("to_unmanaged", &to_unmanaged);
            context.insert("as_unmanaged", &as_unmanaged);
            context.insert("ctors", &ctors);
            context.insert("exception_for_variant", &exception_for_variant);
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
