//! Maps Rust type patterns to C# type patterns.

use crate::lang::TypeId;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::{skip_mapped, try_extract_kind, try_resolve};
use interoptopus::lang;

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
        fallbacks: &model::common::types::fallback::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            skip_mapped!(kinds, id_map, rust_id);

            let rust_pattern = try_extract_kind!(ty, TypePattern);
            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));

            // Determine C# pattern
            #[rustfmt::skip]
            let cs_pattern = match rust_pattern {
                lang::types::TypePattern::CStrPointer => TypePattern::CStrPointer,
                lang::types::TypePattern::Utf8String => TypePattern::Utf8String,
                lang::types::TypePattern::Bool => TypePattern::Bool,
                lang::types::TypePattern::CChar => TypePattern::CChar,
                lang::types::TypePattern::CVoid => TypePattern::CVoid,
                lang::types::TypePattern::Version => TypePattern::Version,

                lang::types::TypePattern::Slice(rust_ty) => TypePattern::Slice(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),
                lang::types::TypePattern::SliceMut(rust_ty) => TypePattern::SliceMut(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),
                lang::types::TypePattern::Vec(rust_ty) => TypePattern::Vec(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),
                lang::types::TypePattern::AsyncCallback(rust_ty) => TypePattern::AsyncCallback(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),
                lang::types::TypePattern::Wire(rust_ty) => TypePattern::Wire(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),

                lang::types::TypePattern::Option(rust_ty) => {
                    let cs_ty = try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty));
                    // Skip ffi::Option wrapping a WireOnly inner type — these only appear
                    // inside Wire<T> structs and are handled by the wire codegen.
                    if is_wire_only_type(*rust_ty, rs_types) {
                        continue;
                    }
                    let Some(TypeKind::DataEnum(data_enum)) = fallbacks.get(cs_id) else { continue };
                    TypePattern::Option(cs_ty, data_enum.clone())
                }

                lang::types::TypePattern::Result(rust_ok, rust_err) => {
                    let cs_ok = try_resolve!(id_map.ty(*rust_ok), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ok));
                    let cs_err = try_resolve!(id_map.ty(*rust_err), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_err));
                    // Skip ffi::Result wrapping a WireOnly inner type.
                    if is_wire_only_type(*rust_ok, rs_types) || is_wire_only_type(*rust_err, rs_types) {
                        continue;
                    }
                    let Some(TypeKind::DataEnum(data_enum)) = fallbacks.get(cs_id) else { continue };
                    TypePattern::Result(cs_ok, cs_err, data_enum.clone())
                }

                // NamedCallback is handled by the delegate kind pass, not here.
                lang::types::TypePattern::NamedCallback(_) => continue,
                lang::types::TypePattern::TaskHandle => TypePattern::TaskHandle,
            };

            kinds.set(cs_id, TypeKind::TypePattern(cs_pattern));
            outcome.changed();
        }

        Ok(outcome)
    }
}

/// Returns `true` if the Rust type is `WireOnly` or is a struct that transitively
/// contains `WireOnly` fields. Such types can only appear inside `Wire<T>`.
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
        lang::types::TypeKind::WireOnly(_) => true,
        lang::types::TypeKind::Struct(s) => s.fields.iter().any(|f| is_wire_only_recursive(f.ty, rs_types, visited)),
        lang::types::TypeKind::TypePattern(lang::types::TypePattern::Option(inner)) => is_wire_only_recursive(*inner, rs_types, visited),
        lang::types::TypeKind::TypePattern(lang::types::TypePattern::Result(ok, err)) => {
            is_wire_only_recursive(*ok, rs_types, visited) || is_wire_only_recursive(*err, rs_types, visited)
        }
        _ => false,
    }
}
