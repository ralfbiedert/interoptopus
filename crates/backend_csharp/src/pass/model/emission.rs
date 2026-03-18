//! Determines the `Emission` target for each C# type and function.
//!
//! Most items inherit their emission from the corresponding Rust inventory entry
//! (e.g., `FileEmission::Common` or `FileEmission::Default`). Items that only
//! exist in the C# model (synthesized by overload passes) default to `Builtin`.
//! Special cases like `TypePattern::Bool` — which is `Builtin` in Rust but must
//! be emitted as a concrete struct in C# — are overridden here.

use crate::lang::meta::{Emission, FileEmission};
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::lang::{FunctionId, TypeId};
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use interoptopus::inventory::{Functions, Types};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    type_emissions: HashMap<TypeId, Emission>,
    fn_emissions: HashMap<FunctionId, Emission>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, type_emissions: HashMap::default(), fn_emissions: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &model::types::kind::Pass,
        id_maps: &model::id_map::Pass,
        rs_types: &Types,
        rs_functions: &Functions,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (type_id, kind) in kinds.iter() {
            if self.type_emissions.contains_key(type_id) {
                continue;
            }

            let emission = lookup_type_emission(*type_id, id_maps, rs_types);

            self.type_emissions.insert(*type_id, emission);
            outcome.changed();
        }

        for (rust_id, rust_fn) in rs_functions {
            let Some(cs_id) = id_maps.fns(*rust_id) else { continue };

            if self.fn_emissions.contains_key(&cs_id) {
                continue;
            }

            self.fn_emissions.insert(cs_id, rust_fn.emission.clone());
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn get_type(&self, ty: TypeId) -> Option<&Emission> {
        self.type_emissions.get(&ty)
    }

    #[must_use]
    pub fn get_fn(&self, fn_id: FunctionId) -> Option<&Emission> {
        self.fn_emissions.get(&fn_id)
    }
}

/// Looks up the Rust emission for a C# type by searching the `id_map` for a matching Rust type.
fn lookup_type_emission(cs_type_id: TypeId, id_maps: &model::id_map::Pass, rs_types: &Types) -> Emission {
    for (rust_id, rust_ty) in rs_types {
        if id_maps.ty(*rust_id) == Some(cs_type_id) {
            return rust_ty.emission.clone();
        }
    }
    // Synthesized types (e.g., overload siblings) won't be in the inventory;
    // default to Builtin so they don't get emitted on their own.
    Emission::Builtin
}
