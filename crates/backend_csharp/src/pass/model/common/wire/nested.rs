//! Registers nested wire-only composite types into the type model.
//!
//! Structs that contain `WireOnly` fields are skipped by the regular struct pass
//! because they have no FFI-safe representation. This pass walks the type graph
//! from each `Wire<T>` inner type, discovers those nested structs, and registers
//! them as `TypeKind::WireOnly(WireOnly::Composite(...))` in the type model so
//! the output passes can emit them like any other type.

use crate::lang::TypeId;
use crate::lang::types::kind::wire::WireOnly as CsWireOnly;
use crate::lang::types::kind::{Composite, Field, TypeKind};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use interoptopus::inventory::{TypeId as RsTypeId, Types as RsTypes};
use interoptopus::lang::meta::Visibility as RsVisibility;
use interoptopus::lang::types::{Repr, Struct, TypeKind as RsTypeKind, WireOnly as RsWireOnly};
use std::collections::HashSet;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    done: bool,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, done: false }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::common::id_map::Pass,
        type_kinds: &mut model::common::types::kind::Pass,
        type_names: &mut model::common::types::names::Pass,
        rs_types: &RsTypes,
    ) -> ModelResult {
        if self.done {
            return Ok(Unchanged);
        }

        let mut outcome = Unchanged;
        let mut registered: HashSet<RsTypeId> = HashSet::new();

        for rust_ty in rs_types.values() {
            let inner_rust_id = match &rust_ty.kind {
                RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Wire(inner)) => *inner,
                _ => continue,
            };

            let Some(inner_rust_ty) = rs_types.get(&inner_rust_id) else { continue };
            let RsTypeKind::Struct(s) = &inner_rust_ty.kind else { continue };

            // Walk the inner struct's type graph to find nested structs with WireOnly fields.
            let mut nested = Vec::new();
            collect_nested_structs(rs_types, s, &mut nested, &mut HashSet::new());
            // Exclude the top-level inner type — it gets its own WireOf* treatment.
            nested.retain(|id| *id != inner_rust_id);

            for nested_id in nested {
                if !registered.insert(nested_id) {
                    continue;
                }

                let Some(cs_id) = id_map.ty(nested_id) else { continue };
                if type_kinds.contains(&cs_id) {
                    continue;
                }

                let Some(nested_ty) = rs_types.get(&nested_id) else { continue };
                let RsTypeKind::Struct(nested_s) = &nested_ty.kind else { continue };

                // Build C# fields, mapping each Rust field type to its C# TypeId.
                let cs_fields: Vec<Field> = nested_s
                    .fields
                    .iter()
                    .filter_map(|f| {
                        let cs_field_ty = id_map.ty(f.ty)?;
                        Some(Field { name: f.name.clone(), docs: f.docs.clone(), visibility: map_visibility(f.visibility), ty: cs_field_ty })
                    })
                    .collect();

                let composite = Composite { fields: cs_fields, repr: Repr::c() };

                type_kinds.set(cs_id, TypeKind::WireOnly(CsWireOnly::Composite(composite)));
                type_names.set(cs_id, nested_ty.name.clone());
                outcome.changed();
            }
        }

        self.done = true;
        Ok(outcome)
    }
}

fn map_visibility(vis: RsVisibility) -> crate::lang::meta::Visibility {
    match vis {
        RsVisibility::Public => crate::lang::meta::Visibility::Public,
        RsVisibility::Private => crate::lang::meta::Visibility::Private,
    }
}

/// Recursively collects struct `TypeId`s reachable from `s` that have `WireOnly` fields.
fn collect_nested_structs(rs_types: &RsTypes, s: &Struct, out: &mut Vec<RsTypeId>, visited: &mut HashSet<RsTypeId>) {
    for f in &s.fields {
        collect_nested_from_type(rs_types, f.ty, out, visited);
    }
}

fn collect_nested_from_type(rs_types: &RsTypes, ty_id: RsTypeId, out: &mut Vec<RsTypeId>, visited: &mut HashSet<RsTypeId>) {
    if !visited.insert(ty_id) {
        return;
    }
    let Some(ty) = rs_types.get(&ty_id) else { return };
    match &ty.kind {
        RsTypeKind::Struct(s) => {
            let has_wire_only = s
                .fields
                .iter()
                .any(|f| rs_types.get(&f.ty).is_some_and(|ft| matches!(&ft.kind, RsTypeKind::WireOnly(_))));
            if has_wire_only {
                out.push(ty_id);
            }
            for f in &s.fields {
                collect_nested_from_type(rs_types, f.ty, out, visited);
            }
        }
        RsTypeKind::WireOnly(RsWireOnly::Vec(inner)) => {
            collect_nested_from_type(rs_types, *inner, out, visited);
        }
        RsTypeKind::WireOnly(RsWireOnly::Map(k, v)) => {
            collect_nested_from_type(rs_types, *k, out, visited);
            collect_nested_from_type(rs_types, *v, out, visited);
        }
        RsTypeKind::Array(arr) => {
            collect_nested_from_type(rs_types, arr.ty, out, visited);
        }
        _ => {}
    }
}
