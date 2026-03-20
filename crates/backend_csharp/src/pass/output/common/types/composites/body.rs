//! Renders composite body definitions using the `body.cs` template.
//!
//! Shared between the Rust and .NET pipelines.

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::{Context, Value};
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
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        struct_class: &model::common::types::info::struct_class::Pass,
        disposable: &model::common::types::info::disposable::Pass,
        managed: &output::common::conversion::unmanaged_conversion::Pass,
        composite_body_unmanaged: &output::common::types::composites::body_unmanaged::Pass,
        composite_body_to_unmanaged: &output::common::types::composites::body_to_unmanaged::Pass,
        composite_body_as_unmanaged: &output::common::types::composites::body_as_unmanaged::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let TypeKind::Composite(composite) = &ty.kind else { continue };

            let name = &ty.name;
            let struct_or_class = if struct_class.is_struct(*type_id) { "struct" } else { "class" };
            let is_disposable = disposable.is_disposable(*type_id).unwrap_or(false);
            let unmanaged = composite_body_unmanaged.get(*type_id).map_or("", std::string::String::as_str);
            let to_unmanaged = composite_body_to_unmanaged.get(*type_id).map_or("", std::string::String::as_str);
            let as_unmanaged = composite_body_as_unmanaged.get(*type_id).map_or("", std::string::String::as_str);

            // Collect disposable fields for the Dispose() method.
            let disposable_fields: Vec<HashMap<&str, Value>> = if is_disposable {
                composite
                    .fields
                    .iter()
                    .filter(|f| disposable.is_disposable(f.ty).unwrap_or(false))
                    .map(|f| {
                        let mut m = HashMap::new();
                        m.insert("name", Value::String(f.name.clone()));
                        m
                    })
                    .collect()
            } else {
                Vec::new()
            };

            // The Marshaller always exposes `ToUnmanaged()`/`ToManaged()` to the runtime,
            // but internally must call the correct method based on conversion category.
            let marshaller_to_unmanaged = managed.to_unmanaged_name(*type_id);
            let marshaller_to_managed = managed.to_managed_name(*type_id);

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("struct_or_class", struct_or_class);
            context.insert("is_disposable", &is_disposable);
            context.insert("unmanaged", &unmanaged);
            context.insert("to_unmanaged", &to_unmanaged);
            context.insert("as_unmanaged", &as_unmanaged);
            context.insert("disposable_fields", &disposable_fields);
            context.insert("marshaller_to_unmanaged", marshaller_to_unmanaged);
            context.insert("marshaller_to_managed", marshaller_to_managed);

            let rendered = templates.render("rust/types/composite/body.cs", &context)?;
            self.composite_body.insert(*type_id, rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.composite_body.get(&type_id)
    }
}
