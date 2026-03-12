//! Renders composite type definitions using the `definition.cs` template.

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    composite_ty: HashMap<TypeId, String>,
}

impl Pass {
    #[must_use] 
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, composite_ty: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        struct_class: &model::types::info::struct_class::Pass,
        unmanaged_names: &output::conversion::unmanaged_names::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let TypeKind::Composite(composite) = &ty.kind else { continue };

            let name = &ty.name;
            let ty = *type_id;
            let struct_or_class = if struct_class.is_struct(ty) { "struct" } else { "class" };

            let fields: Vec<HashMap<&str, String>> = composite
                .fields
                .iter()
                .filter_map(|f| {
                    let unmanaged_name = unmanaged_names.name(f.ty)?;
                    let mut m = HashMap::new();
                    m.insert("name", f.name.clone());
                    m.insert("unmanaged_name", unmanaged_name.clone());
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

    #[must_use] 
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.composite_ty.get(&type_id)
    }
}
