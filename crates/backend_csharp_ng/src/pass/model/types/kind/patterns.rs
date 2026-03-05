//! Maps Rust type patterns to C# type patterns.

use crate::lang::function::Signature;
use crate::lang::types::csharp;
use crate::lang::types::{TypeKind, TypePattern};
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use interoptopus::lang;
use interoptopus::lang::types::TypeInfo;
use std::ffi::c_char;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "model_type_map_patterns" } }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &mut model::id_maps::Pass,
        kinds: &mut model::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            skip_mapped!(id_map, rust_id);
            let rust_pattern = try_extract_kind!(ty, TypePattern);

            // Determine C# TypeId and pattern
            #[rustfmt::skip]
            let (cs_id, cs_pattern) = match rust_pattern {
                // Special C# types with predefined TypeIds
                lang::types::TypePattern::CStrPointer => (csharp::CSTR_PTR, TypePattern::CStrPointer),
                lang::types::TypePattern::Utf8String => (csharp::UTF8_STRING, TypePattern::Utf8String),

                // Simple patterns with derived TypeIds
                lang::types::TypePattern::Bool => (TypeId::from_id(bool::id().id()), TypePattern::Bool),
                lang::types::TypePattern::CChar => (TypeId::from_id(c_char::id().id()), TypePattern::CChar),
                lang::types::TypePattern::CVoid => (TypeId::from_id(<()>::id().id()), TypePattern::CVoid),

                // Patterns with one type parameter
                lang::types::TypePattern::Slice(rust_ty) => (TypeId::from_id(rust_id.id()), TypePattern::Slice(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty)))),
                lang::types::TypePattern::SliceMut(rust_ty) => (TypeId::from_id(rust_id.id()), TypePattern::SliceMut(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty)))),
                lang::types::TypePattern::Vec(rust_ty) => (TypeId::from_id(rust_id.id()), TypePattern::Vec(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty)))),
                lang::types::TypePattern::Option(rust_ty) => (TypeId::from_id(rust_id.id()), TypePattern::Option(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty)))),
                lang::types::TypePattern::AsyncCallback(rust_ty) => (TypeId::from_id(rust_id.id()), TypePattern::AsyncCallback(try_resolve!(id_map.ty(*rust_ty), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ty)))),

                lang::types::TypePattern::APIVersion => (TypeId::from_id(rust_id.id()), TypePattern::ApiVersion),

                // Result pattern with two type parameters
                lang::types::TypePattern::Result(rust_ok, rust_err) => {
                    let cs_ok = try_resolve!(id_map.ty(*rust_ok), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_ok));
                    let cs_err = try_resolve!(id_map.ty(*rust_err), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_err));
                    (TypeId::from_id(rust_id.id()), TypePattern::Result(cs_ok, cs_err))
                }

                // NamedCallback with signature
                lang::types::TypePattern::NamedCallback(rust_sig) => {
                    // Convert return type
                    let cs_rval = try_resolve!(id_map.ty(rust_sig.rval), pass_meta, self.info, crate::pass::MissingItem::RustType(rust_sig.rval));

                    // Convert all arguments
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
                    (TypeId::from_id(rust_id.id()), TypePattern::NamedCallback(cs_sig))
                }

            };

            id_map.set_ty(*rust_id, cs_id);
            kinds.set_kind(cs_id, TypeKind::TypePattern(cs_pattern));
            outcome.changed();
        }

        Ok(outcome)
    }
}
