//! Creates DataEnum types from computed enum variants.

use crate::lang::types::{DataEnum, TypeKind};
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::try_resolve;
use interoptopus::lang;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id_map::Pass,
        kinds: &mut model::types::kind::Pass,
        variants_pass: &model::types::kind::enum_variants::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            match &ty.kind {
                lang::types::TypeKind::Enum(_) => {}
                _ => continue,
            }

            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));
            let variants = try_resolve!(variants_pass.get(cs_id), pass_meta, self.info, crate::pass::MissingItem::CsType(cs_id));

            // Check if we've already processed this type
            if kinds.contains(&cs_id) {
                continue;
            }

            // Create the data enum
            let data_enum = DataEnum { variants: variants.clone() };

            kinds.set(cs_id, TypeKind::DataEnum(data_enum));
            outcome.changed();
        }

        Ok(outcome)
    }
}
