//! Container for all C# types (id → Type), analogous to `fns::all` for functions.
//!
//! Assembles final `Type` instances from TypeKind (via the `kind` pass) and
//! names (via the `names` pass). Other passes should prefer querying this pass
//! over accessing `kind` or `names` directly.

use crate::lang::types::Type;
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::try_resolve;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    types: HashMap<TypeId, Type>,
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
            let name = try_resolve!(names.get(*type_id), pass_meta, self.info, crate::pass::MissingItem::CsType(*type_id));

            // Create the Type
            let ty = Type { name: name.clone(), kind: kind.clone() };

            self.types.insert(*type_id, ty);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn set(&mut self, ty: TypeId, t: Type) {
        self.types.insert(ty, t);
    }

    pub fn get(&self, ty: TypeId) -> Option<&Type> {
        self.types.get(&ty)
    }

pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &Type)> {
        self.types.iter()
    }
}
