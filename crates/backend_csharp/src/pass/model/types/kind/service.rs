//! Maps Rust service types to C# service types.

use crate::lang::types::kind::TypeKind;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::{skip_mapped, try_resolve};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    #[must_use] 
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

            match &ty.kind {
                interoptopus::lang::types::TypeKind::Service => {}
                _ => continue,
            }

            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));

            kinds.set(cs_id, TypeKind::Service);
            outcome.changed();
        }

        Ok(outcome)
    }
}
