//! ...

use crate::lang::types::Field;
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model_id_maps};
use interoptopus::lang;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    fields: HashMap<TypeId, Vec<Field>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: "model_type_map_struct_fields" },
            fields: Default::default(),
        }
    }

    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let rust_struct = match &ty.kind {
                lang::types::TypeKind::Struct(x) => x,
                _ => continue,
            };

            // Create C# TypeId for the struct itself
            let cs_id = TypeId::from_id(rust_id.id());

            // Skip if we've already processed this struct
            if self.fields.contains_key(&cs_id) {
                continue;
            }

            // Try to convert all fields
            let mut cs_fields = Vec::new();
            let mut all_fields_available = true;

            for rust_field in &rust_struct.fields {
                // Look up the C# TypeId for this field's type
                let Some(cs_field_type_id) = id_map.cs_from_rust(rust_field.ty) else {
                    // Field type not yet mapped, skip this struct for now
                    all_fields_available = false;
                    break;
                };

                cs_fields.push(Field {
                    name: rust_field.name.clone(),
                    docs: rust_field.docs.clone(),
                    visibility: map_visibility(rust_field.visibility),
                    ty: cs_field_type_id,
                });
            }

            if !all_fields_available {
                continue;
            }

            // All fields available, register the struct
            id_map.set_rust_to_cs(*rust_id, cs_id);
            self.fields.insert(cs_id, cs_fields);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn get_fields(&self, ty: TypeId) -> Option<&Vec<Field>> {
        self.fields.get(&ty)
    }
}

fn map_visibility(visibility: lang::meta::Visibility) -> crate::lang::meta::Visibility {
    use crate::lang::meta::Visibility as CsVis;
    use interoptopus::lang::meta::Visibility as RsVis;

    match visibility {
        RsVis::Public => CsVis::Public,
        RsVis::Private => CsVis::Private,
    }
}
