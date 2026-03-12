//! Pre-renders custom field conversion snippets for types whose fields need
//! special marshalling (e.g. fixed-size arrays).

use crate::lang::types::TypeKind;
use crate::lang::TypeId;
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

/// Key for a specific field within a type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FieldKey {
    parent: TypeId,
    field_name: String,
}

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    custom_to_managed: HashMap<FieldKey, String>,
    custom_to_unmanaged: HashMap<FieldKey, String>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, custom_to_managed: Default::default(), custom_to_unmanaged: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for (type_id, ty) in types.iter() {
            let composite = match &ty.kind {
                TypeKind::Composite(c) => c,
                _ => continue,
            };

            for f in &composite.fields {
                let Some(field_kind) = types.get(f.ty).map(|t| &t.kind) else {
                    continue;
                };

                match field_kind {
                    TypeKind::Array(a) => {
                        let Some(element_type) = types.get(a.ty).map(|t| &t.name) else {
                            continue;
                        };

                        let mut ctx = Context::new();
                        ctx.insert("field", &f.name);
                        ctx.insert("element_type", element_type.as_str());
                        ctx.insert("len", &a.len);

                        let key = FieldKey { parent: *type_id, field_name: f.name.clone() };

                        if let Ok(rendered) = templates.render("conversion/array_to_managed.cs", &ctx) {
                            self.custom_to_managed.insert(key.clone(), rendered);
                        }
                        if let Ok(rendered) = templates.render("conversion/array_to_unmanaged.cs", &ctx) {
                            self.custom_to_unmanaged.insert(key, rendered);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Returns a pre-rendered custom `ToManaged` snippet for a field, if one exists.
    pub fn custom_to_managed(&self, parent: TypeId, field_name: &str) -> Option<&str> {
        let key = FieldKey { parent, field_name: field_name.to_string() };
        self.custom_to_managed.get(&key).map(|s| s.as_str())
    }

    /// Returns a pre-rendered custom `ToUnmanaged` snippet for a field, if one exists.
    pub fn custom_to_unmanaged(&self, parent: TypeId, field_name: &str) -> Option<&str> {
        let key = FieldKey { parent, field_name: field_name.to_string() };
        self.custom_to_unmanaged.get(&key).map(|s| s.as_str())
    }
}
