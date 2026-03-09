//! Renders composite type definitions using the `definition.cs` template.

use crate::lang::types::{ManagedConversion, TypeKind};
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
        managed_conversion: &model::types::info::managed_conversion::Pass,
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

            let fields: Vec<HashMap<&str, String>> = composite
                .fields
                .iter()
                .filter_map(|f| {
                    let ty_name = names.name(f.ty)?;
                    let unmanaged_name = unmanaged_type_name(ty_name, managed_conversion, f.ty);
                    let mut m = HashMap::new();
                    m.insert("name", f.name.to_string());
                    m.insert("type", unmanaged_name);
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

fn unmanaged_type_name(managed_name: &str, managed_conversion: &model::types::info::managed_conversion::Pass, ty: TypeId) -> String {
    match managed_conversion.managed_conversion(ty) {
        Some(ManagedConversion::AsIs) => managed_name.to_string(),
        Some(_) => format!("{}.Unmanaged", managed_name),
        None => managed_name.to_string(),
    }
}
