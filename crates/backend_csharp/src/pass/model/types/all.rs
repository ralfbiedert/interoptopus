//! Container for all C# types (id → Type), analogous to `fns::all` for functions.
//!
//! Assembles final `Type` instances from `TypeKind` (via the `kind` pass),
//! names (via the `names` pass), and emission targets (via the `emission` pass).
//! Other passes should prefer querying this pass over accessing `kind` or
//! `names` directly.

use crate::lang::TypeId;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::lang::types::{Decorators, MarshalAs, ParamDecorator, RvalDecorator, Type};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::try_resolve;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

#[derive(Debug)]
pub struct Pass {
    info: PassInfo,
    types: HashMap<TypeId, Type>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, types: HashMap::default() }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        kinds: &model::types::kind::Pass,
        names: &model::types::names::Pass,
        emissions: &model::emission::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Iterate through all kinds and create Types
        for (type_id, kind) in kinds.iter() {
            // Skip if we've already created this type
            if self.types.contains_key(type_id) {
                continue;
            }

            let name = try_resolve!(names.get(*type_id), pass_meta, self.info, crate::pass::MissingItem::CsType(*type_id));
            let emission = try_resolve!(emissions.get_type(*type_id), pass_meta, self.info, crate::pass::MissingItem::CsType(*type_id));

            let decorators = match kind {
                TypeKind::TypePattern(TypePattern::CStrPointer) => {
                    Decorators { param: Some(ParamDecorator::MarshalAs(MarshalAs::LPStr)), rval: Some(RvalDecorator::MarshalAs(MarshalAs::LPStr)) }
                }
                _ => Decorators::default(),
            };

            let ty = Type { emission: emission.clone(), name: name.clone(), kind: kind.clone(), decorators };

            self.types.insert(*type_id, ty);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn set(&mut self, ty: TypeId, t: Type) {
        self.types.insert(ty, t);
    }

    #[must_use]
    pub fn get(&self, ty: TypeId) -> Option<&Type> {
        self.types.get(&ty)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &Type)> {
        self.types.iter()
    }
}
