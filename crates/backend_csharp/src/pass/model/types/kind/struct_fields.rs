//! ...

use crate::lang::TypeId;
use crate::lang::types::kind::Field;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::try_extract_kind;
use interoptopus::lang;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    fields: HashMap<TypeId, Vec<Field>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, fields: HashMap::default() }
    }

    pub fn process(&mut self, pass_meta: &mut crate::pass::PassMeta, id_map: &model::id_map::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let rust_struct = try_extract_kind!(ty, Struct);

            // Resolve C# TypeId for the struct
            let Some(cs_id) = id_map.ty(*rust_id) else { continue };

            // Skip if we've already processed this struct
            if self.fields.contains_key(&cs_id) {
                continue;
            }

            // Try to convert all fields
            let mut cs_fields = Vec::new();
            let mut all_fields_available = true;

            for rust_field in &rust_struct.fields {
                // Skip structs with WireOnly fields — they are only used through Wire serialization
                // and should not become C# composite types.
                if let Some(field_rs_ty) = rs_types.get(&rust_field.ty)
                    && matches!(&field_rs_ty.kind, lang::types::TypeKind::WireOnly(_))
                {
                    all_fields_available = false;
                    break;
                }

                // Look up the C# TypeId for this field's type
                let Some(cs_field_type_id) = id_map.ty(rust_field.ty) else {
                    // Field type not yet mapped, skip this struct for now
                    pass_meta.lost_found.missing(self.info, crate::pass::MissingItem::RustType(rust_field.ty));
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
            self.fields.insert(cs_id, cs_fields);
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn get(&self, ty: TypeId) -> Option<&Vec<Field>> {
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
