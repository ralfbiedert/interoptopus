//! Maps Rust type patterns to C# type patterns.

use crate::lang::function::Signature;
use crate::lang::types::{TypeKind, TypePattern};
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::{skip_mapped, try_extract_kind, try_resolve};
use interoptopus::lang;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id::Pass,
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

                lang::types::TypePattern::NamedCallback(rust_sig) => {
                    let cs_rval = try_resolve!(id_map.ty(rust_sig.rval), pass_meta, self.info, crate::pass::MissingItem::RustType(rust_sig.rval));

                    let mut cs_arguments = Vec::new();
                    let mut all_args_available = true;

                    for rust_arg in &rust_sig.arguments {
                        let Some(cs_arg_type) = id_map.ty(rust_arg.ty) else {
                            pass_meta.lost_found.missing(self.info, crate::pass::MissingItem::RustType(rust_arg.ty));
                            all_args_available = false;
                            break;
                        };

                        cs_arguments.push(crate::lang::function::Argument { name: rust_arg.name.clone(), ty: cs_arg_type });
                    }

                    if !all_args_available {
                        continue;
                    }

                    let cs_sig = Signature { arguments: cs_arguments, rval: cs_rval };
                    TypePattern::NamedCallback(cs_sig)
                }
            };

            kinds.set_kind(cs_id, TypeKind::TypePattern(cs_pattern));
            outcome.changed();
        }

        Ok(outcome)
    }
}
