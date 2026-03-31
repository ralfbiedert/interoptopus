//! Renders composite type definitions using the `definition.cs` template.
//!
//! Shared between the Rust and .NET pipelines. Each composite gets a `partial struct`
//! (or `partial class`) declaration listing its managed-side public fields.

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{OutputResult, PassInfo, format_docs, model, output};
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
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        struct_class: &model::common::types::info::struct_class::Pass,
        disposable: &model::common::types::info::disposable::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let TypeKind::Composite(composite) = &ty.kind else { continue };

            let name = &ty.name;
            let struct_or_class = if struct_class.is_struct(*type_id) { "struct" } else { "class" };
            let is_disposable = disposable.is_disposable(*type_id).unwrap_or(false);

            let fields: Vec<HashMap<&str, String>> = composite
                .fields
                .iter()
                .filter_map(|f| {
                    let managed_name = types.get(f.ty).map(|t| t.name.clone())?;
                    let mut m = HashMap::new();
                    m.insert("name", f.name.clone());
                    m.insert("managed_name", managed_name);
                    m.insert("docs", format_docs(&f.docs.lines));
                    Some(m)
                })
                .collect();

            let docs = format_docs(&ty.docs);
            let visibility = ty.visibility.to_string();

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("fields", &fields);
            context.insert("docs", &docs);
            context.insert("is_disposable", &is_disposable);
            context.insert("visibility", &visibility);

            let rendered = templates.render("common/types/composite/definition.cs", &context)?;
            self.composite_ty.insert(*type_id, rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.composite_ty.get(&type_id)
    }
}
