//! Container for all C# types (id → Type), analogous to `fns::all` for functions.
//!
//! Assembles final `Type` instances from `TypeKind` (via the `kind` pass) and
//! names (via the `names` pass). Other passes should prefer querying this pass
//! over accessing `kind` or `names` directly.

use crate::lang::TypeId;
use crate::lang::meta::Emission;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::lang::types::{Decorators, MarshalAs, ParamDecorator, RvalDecorator, Type};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::try_resolve;
use interoptopus::inventory::Types;
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
        kinds: &model::common::types::kind::Pass,
        names: &model::common::types::names::Pass,
        id_maps: &model::common::id_map::Pass,
        rs_types: &Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Iterate through all kinds and create Types
        for (type_id, kind) in kinds.iter() {
            // Skip if we've already created this type
            if self.types.contains_key(type_id) {
                continue;
            }

            // Get the name for this type
            let name = try_resolve!(names.get(*type_id), pass_meta, self.info, crate::pass::MissingItem::CsType(*type_id));

            let emission = lookup_emission(*type_id, id_maps, rs_types);

            // Create the Type
            let decorators = match kind {
                TypeKind::TypePattern(TypePattern::CStrPointer) => {
                    Decorators { param: Some(ParamDecorator::MarshalAs(MarshalAs::LPStr)), rval: Some(RvalDecorator::MarshalAs(MarshalAs::LPStr)) }
                }
                _ => Decorators::default(),
            };

            let docs = lookup_docs(*type_id, id_maps, rs_types);
            let ty = Type { emission, name: name.clone(), docs, kind: kind.clone(), decorators };

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

/// Looks up the Rust emission for a C# type by searching the `id_map` for a matching Rust type.
fn lookup_emission(cs_type_id: TypeId, id_maps: &model::common::id_map::Pass, rs_types: &Types) -> Emission {
    // Try to find the corresponding Rust type by checking all Rust types
    for (rust_id, rust_ty) in rs_types {
        if id_maps.ty(*rust_id) == Some(cs_type_id) {
            return rust_ty.emission.clone();
        }
    }

    // Synthesized types (e.g., overload siblings) won't be in the inventory;
    // default to Builtin so they don't get emitted on their own.
    Emission::Builtin
}

/// Looks up the Rust docs for a C# type by searching the `id_map` for a matching Rust type.
fn lookup_docs(cs_type_id: TypeId, id_maps: &model::common::id_map::Pass, rs_types: &Types) -> Vec<String> {
    for (rust_id, rust_ty) in rs_types {
        if id_maps.ty(*rust_id) == Some(cs_type_id) {
            return rust_ty.docs.lines.clone();
        }
    }
    Vec::new()
}
