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
        id_map: &model::id_map::Pass,
        kinds: &mut model::types::kind::Pass,
        fallbacks: &model::types::fallback::Pass,
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
                lang::types::TypePattern::APIVersion => TypePattern::ApiVersion,

                lang::types::TypePattern::Slice(rust_ty) => TypePattern::Slice(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),
                lang::types::TypePattern::SliceMut(rust_ty) => TypePattern::SliceMut(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),
                lang::types::TypePattern::Vec(rust_ty) => TypePattern::Vec(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),
                lang::types::TypePattern::AsyncCallback(rust_ty) => TypePattern::AsyncCallback(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),
                lang::types::TypePattern::Wire(rust_ty) => TypePattern::Wire(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty))),

                lang::types::TypePattern::Option(rust_ty) => {
                    let cs_ty = try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty));
                    let Some(TypeKind::DataEnum(data_enum)) = fallbacks.get(cs_id) else { continue };
                    TypePattern::Option(cs_ty, data_enum.clone())
                }

                lang::types::TypePattern::Result(rust_ok, rust_err) => {
                    let cs_ok = try_resolve!(id_map.ty(*rust_ok), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ok));
                    let cs_err = try_resolve!(id_map.ty(*rust_err), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_err));
                    let Some(TypeKind::DataEnum(data_enum)) = fallbacks.get(cs_id) else { continue };
                    TypePattern::Result(cs_ok, cs_err, data_enum.clone())
                }

                // NamedCallback is handled by the delegate kind pass, not here.
                lang::types::TypePattern::NamedCallback(_) => continue,
            };

            kinds.set(cs_id, TypeKind::TypePattern(cs_pattern));
            outcome.changed();
        }

        Ok(outcome)
    }
}
