//! Creates `ByRef` and `ByOut` sibling types for each existing `IntPtr` pointer type.
//!
//! For every `Pointer::IntPtr(pointee, _)` that is fully resolved in the `map` pass,
//! this pass creates two new types — `Pointer::ByRef(pointee)` and `Pointer::ByOut(pointee)` —
//! with fresh TypeIds derived from the original. It registers them in the kind, name,
//! and map passes, and maintains a family lookup so other passes can find all siblings.

use crate::lang::types::{Pointer, PointerKind, Type, TypeKind};
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;
use std::sync::Arc;

/// The IntPtr/ByRef/ByOut family for a single pointer type.
#[derive(Debug, Clone)]
pub struct Family {
    pub intptr: TypeId,
    pub by_ref: TypeId,
    pub by_out: TypeId,
}

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    /// Maps any member TypeId (intptr, by_ref, or by_out) to its family.
    families: HashMap<TypeId, Arc<Family>>,
    /// Tracks which IntPtr types we've already processed.
    processed: HashMap<TypeId, ()>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, families: Default::default(), processed: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &mut model::types::kind::Pass,
        names: &mut model::types::names::Pass,
        types: &mut model::types::all::Pass,
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
            if self.processed.contains_key(&intptr_id) {
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

            let intptr_name = intptr_type.name.clone();
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

            // Register in the map pass so they're fully resolved
            types.set(by_ref_id, Type { name: format!("ref {pointee_name}"), kind: TypeKind::Pointer(Pointer { kind: PointerKind::ByRef, target: pointee_id }) });
            types.set(by_out_id, Type { name: format!("out {pointee_name}"), kind: TypeKind::Pointer(Pointer { kind: PointerKind::ByOut, target: pointee_id }) });

            // Build family
            let family = Arc::new(Family { intptr: intptr_id, by_ref: by_ref_id, by_out: by_out_id });

            self.families.insert(intptr_id, Arc::clone(&family));
            self.families.insert(by_ref_id, Arc::clone(&family));
            self.families.insert(by_out_id, family);

            self.processed.insert(intptr_id, ());
            outcome.changed();
        }

        Ok(outcome)
    }

    /// Look up the pointer family for any member TypeId (intptr, by_ref, or by_out).
    pub fn get(&self, type_id: TypeId) -> Option<&Arc<Family>> {
        self.families.get(&type_id)
    }
}
