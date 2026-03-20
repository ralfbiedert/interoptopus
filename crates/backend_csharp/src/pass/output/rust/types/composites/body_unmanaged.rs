//! Renders the `Unmanaged` struct for each composite using the `body_unmanaged.cs` template.

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus::lang::types::Layout;
use interoptopus_backends::template::{Context, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    composite_body_unmanaged: HashMap<TypeId, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, composite_body_unmanaged: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        managed: &output::rust::conversion::unmanaged_conversion::Pass,
        unmanaged_names: &output::rust::conversion::unmanaged_names::Pass,
        field_conversions: &output::rust::conversion::fields::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let TypeKind::Composite(composite) = &ty.kind else { continue };

            let name = &ty.name;

            let fields: Vec<HashMap<&str, Value>> = composite
                .fields
                .iter()
                .map(|f| {
                    let mut m = HashMap::new();
                    m.insert("name", Value::String(f.name.clone()));

                    // Arrays need special `fixed` buffer syntax in unmanaged structs.
                    if let Some(TypeKind::Array(arr)) = types.get(f.ty).map(|t| &t.kind) {
                        let element_name = types.get(arr.ty).map_or("byte", |t| t.name.as_str());
                        m.insert("is_fixed_array", Value::Bool(true));
                        m.insert("element_type", Value::String(element_name.to_string()));
                        m.insert("len", Value::from(arr.len as u64));
                    } else {
                        let ty_name = unmanaged_names
                            .name(f.ty)
                            .cloned()
                            .unwrap_or_else(|| types.get(f.ty).map(|t| t.name.clone()).unwrap_or_default());
                        m.insert("type", Value::String(ty_name));
                    }

                    let to_managed = managed.to_managed_suffix(f.ty).to_string();
                    m.insert("to_managed", Value::String(to_managed));
                    if let Some(custom) = field_conversions.custom_to_managed(*type_id, &f.name) {
                        m.insert("custom_to_managed", Value::String(custom.to_string()));
                    }
                    m
                })
                .collect();

            let to_managed_method = managed.to_managed_name(*type_id);

            let is_packed = composite.repr.layout == Layout::Packed;

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("to_managed_method", to_managed_method);
            context.insert("fields", &fields);
            context.insert("is_packed", &is_packed);

            let rendered = templates.render("rust/types/composite/body_unmanaged.cs", &context)?;
            self.composite_body_unmanaged.insert(*type_id, rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.composite_body_unmanaged.get(&type_id)
    }
}
