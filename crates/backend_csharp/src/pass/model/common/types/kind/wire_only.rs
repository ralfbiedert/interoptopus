//! Maps Rust `WireOnly` types to C# `WireOnly` type kinds.
//!
//! Rust types with `TypeKind::WireOnly` (e.g., `String`, `Vec<T>`, `HashMap<K,V>`,
//! `Option<T>`) appear in the inventory but are not handled by the regular struct,
//! enum, or pattern kind passes. This pass gives them a C# `TypeKind::WireOnly`
//! entry so that downstream passes (names, `managed_conversion`, etc.) can reference
//! them without panicking.

use crate::lang::types::kind::TypeKind;
use crate::lang::types::kind::wire::WireOnly as CsWireOnly;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::{skip_mapped, try_resolve};
use interoptopus::lang::types::WireOnly as RsWireOnly;

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
        id_map: &model::common::id_map::Pass,
        kinds: &mut model::common::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            skip_mapped!(kinds, id_map, rust_id);

            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));

            let cs_kind = match &ty.kind {
                interoptopus::lang::types::TypeKind::WireOnly(rs_wire) => match rs_wire {
                    RsWireOnly::String => TypeKind::WireOnly(CsWireOnly::String),
                    RsWireOnly::Vec(inner) => {
                        let cs_inner = try_resolve!(id_map.ty(*inner), pass_meta, self.info, crate::pass::MissingItem::RustType(*inner));
                        TypeKind::WireOnly(CsWireOnly::Vec(cs_inner))
                    }
                    RsWireOnly::Map(k, v) => {
                        let cs_k = try_resolve!(id_map.ty(*k), pass_meta, self.info, crate::pass::MissingItem::RustType(*k));
                        let cs_v = try_resolve!(id_map.ty(*v), pass_meta, self.info, crate::pass::MissingItem::RustType(*v));
                        TypeKind::WireOnly(CsWireOnly::Map(cs_k, cs_v))
                    }
                    RsWireOnly::Option(inner) => {
                        let cs_inner = try_resolve!(id_map.ty(*inner), pass_meta, self.info, crate::pass::MissingItem::RustType(*inner));
                        TypeKind::WireOnly(CsWireOnly::Nullable(cs_inner))
                    }
                },
                // ffi::Option<T> or ffi::Result<T, E> wrapping a wire-only inner type.
                // The patterns pass skips these; give them a WireOnly kind so they
                // don't leave dangling references in the type model.
                interoptopus::lang::types::TypeKind::TypePattern(interoptopus::lang::types::TypePattern::Option(inner)) if is_wire_only_type(*inner, rs_types) => {
                    let cs_inner = try_resolve!(id_map.ty(*inner), pass_meta, self.info, crate::pass::MissingItem::RustType(*inner));
                    TypeKind::WireOnly(CsWireOnly::Nullable(cs_inner))
                }
                interoptopus::lang::types::TypeKind::TypePattern(interoptopus::lang::types::TypePattern::Result(ok, _err)) if is_wire_only_type(*ok, rs_types) => {
                    let cs_ok = try_resolve!(id_map.ty(*ok), pass_meta, self.info, crate::pass::MissingItem::RustType(*ok));
                    TypeKind::WireOnly(CsWireOnly::Nullable(cs_ok))
                }
                _ => continue,
            };

            kinds.set(cs_id, cs_kind);
            outcome.changed();
        }

        Ok(outcome)
    }
}

/// Returns `true` if the Rust type is `WireOnly` or is a struct that transitively
/// contains `WireOnly` fields.
fn is_wire_only_type(ty_id: interoptopus::inventory::TypeId, rs_types: &interoptopus::inventory::Types) -> bool {
    let mut visited = std::collections::HashSet::new();
    is_wire_only_recursive(ty_id, rs_types, &mut visited)
}

fn is_wire_only_recursive(
    ty_id: interoptopus::inventory::TypeId,
    rs_types: &interoptopus::inventory::Types,
    visited: &mut std::collections::HashSet<interoptopus::inventory::TypeId>,
) -> bool {
    if !visited.insert(ty_id) {
        return false;
    }
    let Some(ty) = rs_types.get(&ty_id) else { return false };
    match &ty.kind {
        interoptopus::lang::types::TypeKind::WireOnly(_) => true,
        interoptopus::lang::types::TypeKind::Struct(s) => s.fields.iter().any(|f| is_wire_only_recursive(f.ty, rs_types, visited)),
        interoptopus::lang::types::TypeKind::TypePattern(interoptopus::lang::types::TypePattern::Option(inner)) => is_wire_only_recursive(*inner, rs_types, visited),
        interoptopus::lang::types::TypeKind::TypePattern(interoptopus::lang::types::TypePattern::Result(ok, err)) => {
            is_wire_only_recursive(*ok, rs_types, visited) || is_wire_only_recursive(*err, rs_types, visited)
        }
        _ => false,
    }
}
