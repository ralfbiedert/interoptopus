//! Renders enum body definitions using the `enum_body.cs` template.

use crate::lang::TypeId;
use crate::lang::types::kind::{DataEnum, TypeKind, TypePattern, Variant};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::{Context, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enum_body: HashMap<TypeId, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, enum_body: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        struct_class: &model::common::types::info::struct_class::Pass,
        disposable: &model::common::types::info::disposable::Pass,
        enum_body_unmanaged_variant: &output::common::types::enums::body_unmanaged_variant::Pass,
        enum_body_unmanaged: &output::common::types::enums::body_unmanaged::Pass,
        enum_body_to_unmanaged: &output::common::types::enums::body_to_unmanaged::Pass,
        enum_body_as_unmanaged: &output::common::types::enums::body_as_unmanaged::Pass,
        enum_body_ctors: &output::common::types::enums::body_ctors::Pass,
        enum_body_from_call: &output::common::types::enums::body_from_call::Pass,
        enum_body_exception_for_variant: &output::common::types::enums::body_exception_for_variant::Pass,
        enum_body_tostring: &output::common::types::enums::body_tostring::Pass,
        managed: &output::common::conversion::unmanaged_conversion::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let type_kind = &ty.kind;
            match type_kind {
                TypeKind::DataEnum(_) => {}
                TypeKind::TypePattern(TypePattern::Result(_, _, _)) => {}
                TypeKind::TypePattern(TypePattern::Option(_, _)) => {}
                _ => continue,
            }

            let name = &ty.name;
            let visibility = ty.visibility.to_string();

            // Managed-only types (e.g. Result<Service, Error> siblings) have no Unmanaged representation.
            let is_managed_only = match type_kind {
                TypeKind::TypePattern(TypePattern::Result(ok_ty, _, _) | TypePattern::Option(ok_ty, _)) => {
                    types.get(*ok_ty).is_some_and(|t| matches!(&t.kind, TypeKind::Service))
                }
                _ => false,
            };

            let ty = *type_id;
            let struct_or_class = if struct_class.is_struct(ty) { "struct" } else { "class" };
            let is_disposable = disposable.is_disposable(*type_id).unwrap_or(false);

            let unmanaged_variants = enum_body_unmanaged_variant.get(*type_id).unwrap_or(&[]);
            let unmanaged = enum_body_unmanaged.get(*type_id).map_or("", std::string::String::as_str);
            let to_unmanaged = enum_body_to_unmanaged.get(*type_id).map_or("", std::string::String::as_str);
            let as_unmanaged = enum_body_as_unmanaged.get(*type_id).map_or("", std::string::String::as_str);
            let ctors = enum_body_ctors.get(*type_id).map_or("", std::string::String::as_str);
            let from_call = enum_body_from_call.get(*type_id).map_or("", std::string::String::as_str);
            let exception_for_variant = enum_body_exception_for_variant.get(*type_id).map_or("", std::string::String::as_str);
            let to_string = enum_body_tostring.get(*type_id).map_or("", std::string::String::as_str);

            // Collect disposable variant fields for the Dispose() method.
            let disposable_variants: Vec<HashMap<&str, Value>> = if is_disposable {
                let variants: &[Variant] = match type_kind {
                    TypeKind::DataEnum(de) => &de.variants,
                    TypeKind::TypePattern(TypePattern::Option(_, de)) => &de.variants,
                    TypeKind::TypePattern(TypePattern::Result(_, _, de)) => &de.variants,
                    _ => &[],
                };
                variants
                    .iter()
                    .filter(|v| v.ty.is_some_and(|ty| disposable.is_disposable(ty).unwrap_or(false)))
                    .map(|v| {
                        let mut m = HashMap::new();
                        m.insert("name", Value::String(format!("_{}", v.name)));
                        m.insert("tag", Value::from(v.tag as i64));
                        m
                    })
                    .collect()
            } else {
                Vec::new()
            };

            let marshaller_to_unmanaged = managed.to_unmanaged_name(*type_id);
            let marshaller_to_managed = managed.to_managed_name(*type_id);

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("is_disposable", &is_disposable);
            context.insert("is_managed_only", &is_managed_only);
            context.insert("visibility", &visibility);
            context.insert("disposable_variants", &disposable_variants);
            context.insert("unmanaged_variants", &unmanaged_variants);
            context.insert("unmanaged", &unmanaged);
            context.insert("to_unmanaged", &to_unmanaged);
            context.insert("as_unmanaged", &as_unmanaged);
            context.insert("ctors", &ctors);
            context.insert("from_call", &from_call);
            context.insert("exception_for_variant", &exception_for_variant);
            context.insert("to_string", &to_string);
            context.insert("marshaller_to_unmanaged", marshaller_to_unmanaged);
            context.insert("marshaller_to_managed", marshaller_to_managed);

            let rendered = templates.render("common/types/enums/body.cs", &context)?;
            self.enum_body.insert(*type_id, rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.enum_body.get(&type_id)
    }
}
