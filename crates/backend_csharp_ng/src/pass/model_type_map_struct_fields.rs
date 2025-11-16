//! ...

use crate::lang::types::Field;
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, model_id_maps};
use interoptopus::lang;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    fields: HashMap<TypeId, Vec<Field>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { fields: Default::default() }
    }

    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        for (rust_id, ty) in rs_types {
            let cs_fields = match &ty.kind {
                lang::types::TypeKind::Struct(x) => {
                    // Convert each Rust field to a C# field
                    x.fields
                        .iter()
                        .map(|rust| {
                            // Convert the field's type ID from Rust to C#
                            let cs_field_type_id = TypeId::from_id(rust.ty.id());

                            Field { name: rust.name.clone(), docs: rust.docs.clone(), visibility: map_visibility(rust.visibility), ty: cs_field_type_id }
                        })
                        .collect()
                }
                _ => continue,
            };

            let cs_id = TypeId::from_id(rust_id.id());
            id_map.set_rust_to_cs(*rust_id, cs_id);
            self.fields.insert(cs_id, cs_fields);
        }

        Ok(Unchanged)
    }
}

fn map_visibility(visibility: interoptopus::lang::meta::Visibility) -> crate::lang::meta::Visibility {
    use crate::lang::meta::Visibility as CsVis;
    use interoptopus::lang::meta::Visibility as RsVis;

    match visibility {
        RsVis::Public => CsVis::Public,
        RsVis::Private => CsVis::Private,
    }
}
