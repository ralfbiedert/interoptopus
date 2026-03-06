//! ...

use crate::lang::types::Variant;
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::try_extract_kind;
use interoptopus::lang;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    variants: HashMap<TypeId, Vec<Variant>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, variants: Default::default() }
    }

    pub fn process(&mut self, pass_meta: &mut crate::pass::PassMeta, id_map: &mut model::id::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let rust_enum = try_extract_kind!(ty, Enum);

            // Create C# TypeId for the enum itself
            let cs_id = TypeId::from_id(rust_id.id());

            // Skip if we've already processed this enum
            if self.variants.contains_key(&cs_id) {
                continue;
            }

            // Try to convert all variants
            let mut cs_variants = Vec::new();
            let mut all_variants_available = true;

            for (index, rust_variant) in rust_enum.variants.iter().enumerate() {
                let (tag, cs_variant_type_id) = match &rust_variant.kind {
                    lang::types::VariantKind::Unit(tag) => (*tag, None),
                    lang::types::VariantKind::Tuple(rust_type_id) => {
                        // Tuple variant: use index as tag, look up the C# TypeId
                        let Some(cs_type_id) = id_map.ty(*rust_type_id) else {
                            // Variant type not yet mapped, skip this enum for now
                            pass_meta.lost_found.missing(self.info, crate::pass::MissingItem::RustType(*rust_type_id));
                            all_variants_available = false;
                            break;
                        };
                        (index, Some(cs_type_id))
                    }
                };

                cs_variants.push(Variant { name: rust_variant.name.clone(), docs: rust_variant.docs.clone(), tag, ty: cs_variant_type_id });
            }

            if !all_variants_available {
                continue;
            }

            // All variants available, register the enum
            id_map.set_ty(*rust_id, cs_id);
            self.variants.insert(cs_id, cs_variants);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn get_variants(&self, ty: TypeId) -> Option<&Vec<Variant>> {
        self.variants.get(&ty)
    }
}
