//! Creates `ByRef` and `ByOut` sibling types for each existing `IntPtr` pointer type.
//!
//! For every `Pointer::IntPtr(pointee, _)` that is fully resolved in the `all` pass,
//! this pass creates two new types — `Pointer::ByRef(pointee)` and `Pointer::ByOut(pointee)` —
//! with fresh `TypeIds` derived from the original. It registers them in the kind, name,
//! and all passes, and registers the family in the overload all pass.

use crate::lang::TypeId;
use crate::lang::types::kind::{Pointer, PointerKind, TypeKind};
use crate::lang::types::{OverloadFamily, PointerFamily, Type};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    /// Tracks which `IntPtr` types we've already processed.
    processed: HashSet<TypeId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, processed: HashSet::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &mut model::types::kind::Pass,
        names: &mut model::types::names::Pass,
        types: &mut model::types::all::Pass,
        overloads: &mut model::types::overload::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect IntPtr types that are fully resolved in the map pass
        let intptr_types: Vec<(TypeId, TypeId)> = kinds
            .iter()
            .filter_map(|(&type_id, kind)| match kind {
                TypeKind::Pointer(Pointer { kind: PointerKind::IntPtr(_), target }) => Some((type_id, *target)),
                _ => None,
            })
            .collect();

        for (intptr_id, pointee_id) in intptr_types {
            if self.processed.contains(&intptr_id) {
                continue;
            }

            // Wait until the IntPtr type is fully resolved in the map pass
            let Some(intptr_type) = types.get(intptr_id) else {
                continue;
            };

            // Also need the pointee to be named
            let Some(pointee_name) = names.get(pointee_id) else {
                continue;
            };

            let pointee_name = pointee_name.clone();

            // Derive new TypeIds for ByRef and ByOut variants
            let by_ref_id = TypeId::from_id(intptr_id.id().derive(0x_6279_7265_665F_7369)); // "byref_si"
            let by_out_id = TypeId::from_id(intptr_id.id().derive(0x_6279_6F75_745F_7369)); // "byout_si"

            // Register kinds
            kinds.set(by_ref_id, TypeKind::Pointer(Pointer { kind: PointerKind::ByRef, target: pointee_id }));
            kinds.set(by_out_id, TypeKind::Pointer(Pointer { kind: PointerKind::ByOut, target: pointee_id }));

            // Register names
            names.set(by_ref_id, format!("ref {pointee_name}"));
            names.set(by_out_id, format!("out {pointee_name}"));

            // Register in the all pass so they're fully resolved
            types.set(by_ref_id, Type { name: format!("ref {pointee_name}"), kind: TypeKind::Pointer(Pointer { kind: PointerKind::ByRef, target: pointee_id }) });
            types.set(by_out_id, Type { name: format!("out {pointee_name}"), kind: TypeKind::Pointer(Pointer { kind: PointerKind::ByOut, target: pointee_id }) });

            // Register family in the overload all pass
            let family = Arc::new(OverloadFamily::Pointer(PointerFamily { intptr: intptr_id, by_ref: by_ref_id, by_out: by_out_id }));

            overloads.register(intptr_id, Arc::clone(&family));
            overloads.register(by_ref_id, Arc::clone(&family));
            overloads.register(by_out_id, family);

            self.processed.insert(intptr_id);
            outcome.changed();
        }

        Ok(outcome)
    }
}
