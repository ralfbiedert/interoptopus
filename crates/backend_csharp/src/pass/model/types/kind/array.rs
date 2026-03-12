//! Maps Rust arrays to C# arrays.

use crate::lang::types::{Array, TypeKind};
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::{skip_mapped, try_extract_kind, try_resolve};

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
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            skip_mapped!(kinds, id_map, rust_id);
            let rust_array = try_extract_kind!(ty, Array);
            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));

            let cs_element_type = try_resolve!(id_map.ty(rust_array.ty), pass_meta, self.info, crate::pass::MissingItem::RustType(rust_array.ty));

            // Create the C# array with mapped element type
            let cs_array = Array { ty: cs_element_type, len: rust_array.len };

            kinds.set(cs_id, TypeKind::Array(cs_array));
            outcome.changed();
        }

        Ok(outcome)
    }
}
