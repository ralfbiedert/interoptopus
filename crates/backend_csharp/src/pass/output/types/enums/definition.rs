//! Renders enum type definitions using the `enum_ty.cs` template.

use crate::lang::types::{TypeKind, TypePattern};
use crate::lang::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enum_ty: HashMap<TypeId, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, enum_ty: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        struct_class: &model::types::info::struct_class::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let type_kind = &ty.kind;
            let data_enum = match type_kind {
                TypeKind::DataEnum(e) => e,
                TypeKind::TypePattern(TypePattern::Result(_, _, e)) => e,
                TypeKind::TypePattern(TypePattern::Option(_, e)) => e,
                _ => continue,
            };

            let name = &ty.name;

            let ty = *type_id;
            let struct_or_class = if struct_class.is_struct(ty) { "struct" } else { "class" };

            let variants: Vec<HashMap<&str, String>> = data_enum
                .variants
                .iter()
                .filter_map(|v| {
                    let ty = v.ty?;
                    let ty_name = types.get(ty).map(|t| &t.name)?;
                    let mut m = HashMap::new();
                    m.insert("name", v.name.to_string());
                    m.insert("type", ty_name.clone());
                    Some(m)
                })
                .collect();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("variants", &variants);

            let rendered = templates.render("types/enums/definition.cs", &context)?;
            self.enum_ty.insert(*type_id, rendered);
        }

        Ok(())
    }

    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.enum_ty.get(&type_id)
    }
}
