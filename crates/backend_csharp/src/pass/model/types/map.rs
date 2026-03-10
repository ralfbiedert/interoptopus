//! Creates the final Type instances from TypeKind and names.

use crate::lang::types::Type;
use crate::lang::{TypeId, Types};
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::try_resolve;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    types: Types,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, types: Default::default() }
    }

    pub fn process(&mut self, pass_meta: &mut crate::pass::PassMeta, kinds: &model::types::kind::Pass, names: &model::types::names::Pass) -> ModelResult {
        let mut outcome = Unchanged;

        // Iterate through all kinds and create Types
        for (type_id, kind) in kinds.iter() {
            // Skip if we've already created this type
            if self.types.contains_key(type_id) {
                continue;
            }

            // Get the name for this type
            let name = try_resolve!(names.name(*type_id), pass_meta, self.info, crate::pass::MissingItem::CsType(*type_id));

            // Create the Type
            let ty = Type { name: name.clone(), kind: kind.clone() };

            self.types.insert(*type_id, ty);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn ty(&self, ty: TypeId) -> Option<&Type> {
        self.types.get(&ty)
    }
}
