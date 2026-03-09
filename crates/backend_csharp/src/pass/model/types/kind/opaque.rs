//! Maps opaque types

use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::skip_mapped;

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
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &mut model::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            skip_mapped!(kinds, rust_id);

            match &ty.kind {
                interoptopus::lang::types::TypeKind::Opaque => {}
                _ => continue,
            }

            let cs_id = TypeId::from_id(rust_id.id());

            // Register the opaque type (no dependencies to check)
            kinds.set_kind(cs_id, TypeKind::Opaque);
            outcome.changed();
        }

        Ok(outcome)
    }
}
