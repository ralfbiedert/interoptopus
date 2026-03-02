//! Maps Rust type patterns to C# type patterns.

use crate::lang::function::Signature;
use crate::lang::types::csharp;
use crate::lang::types::{TypeKind, TypePattern};
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model_id_maps, model_type_kinds, ModelResult, PassInfo};
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
        pass_meta: &mut super::PassMeta,
        id_map: &mut model_id_maps::Pass,
        kinds: &mut model_type_kinds::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        macro_rules! resolve1 {
            ($rust_ty:expr, $variant:ident) => {{
                let Some(cs_ty) = id_map.ty(*$rust_ty) else {
                    pass_meta.lost_found.missing(self.info, super::MissingItem::RustType(*$rust_ty));
                    continue;
                };
                (TypeId::from_id(rust_id.id()), TypePattern::$variant(cs_ty))
            }};
        }

        macro_rules! resolve2 {
            ($a:expr, $b:expr, $variant:ident) => {{
                let Some(cs_a) = id_map.ty(*$a) else {
                    pass_meta.lost_found.missing(self.info, super::MissingItem::RustType(*$a));
                    continue;
                };
                let Some(cs_b) = id_map.ty(*$b) else {
                    pass_meta.lost_found.missing(self.info, super::MissingItem::RustType(*$b));
                    continue;
                };
                (TypeId::from_id(rust_id.id()), TypePattern::$variant(cs_a, cs_b))
            }};
        }

        for (rust_id, ty) in rs_types {
            let rust_pattern = match &ty.kind {
                lang::types::TypeKind::TypePattern(pattern) => pattern,
                _ => continue,
            };

            // Check if we already processed this pattern
            if id_map.ty(*rust_id).is_some() {
                continue;
            }

            // Determine C# TypeId and pattern
            let (cs_id, cs_pattern) = match rust_pattern {
                // Special C# types with predefined TypeIds
                lang::types::TypePattern::CStrPointer => (csharp::CSTR_PTR, TypePattern::CStrPointer),
                lang::types::TypePattern::Utf8String => (csharp::UTF8_STRING, TypePattern::Utf8String),

                // Simple patterns with derived TypeIds
                lang::types::TypePattern::Bool => (TypeId::from_id(bool::id().id()), TypePattern::Bool),
                lang::types::TypePattern::CChar => (TypeId::from_id(c_char::id().id()), TypePattern::CChar),
                lang::types::TypePattern::CVoid => (TypeId::from_id(<()>::id().id()), TypePattern::CVoid),

                // Patterns with one type parameter
                lang::types::TypePattern::Slice(rust_ty) => resolve1!(rust_ty, Slice),
                lang::types::TypePattern::SliceMut(rust_ty) => resolve1!(rust_ty, SliceMut),
                lang::types::TypePattern::Vec(rust_ty) => resolve1!(rust_ty, Vec),
                lang::types::TypePattern::Option(rust_ty) => resolve1!(rust_ty, Option),
                lang::types::TypePattern::AsyncCallback(rust_ty) => resolve1!(rust_ty, AsyncCallback),

                // Result pattern with two type parameters
                lang::types::TypePattern::Result(rust_ok, rust_err) => resolve2!(rust_ok, rust_err, Result),

                // NamedCallback with signature
                lang::types::TypePattern::NamedCallback(rust_sig) => {
                    // Convert return type
                    let Some(cs_rval) = id_map.ty(rust_sig.rval) else {
                        pass_meta.lost_found.missing(self.info, super::MissingItem::RustType(rust_sig.rval));
                        continue;
                    };

                    // Convert all arguments
                    let mut cs_arguments = Vec::new();
                    let mut all_args_available = true;

                    for rust_arg in &rust_sig.arguments {
                        let Some(cs_arg_type) = id_map.ty(rust_arg.ty) else {
                            pass_meta.lost_found.missing(self.info, super::MissingItem::RustType(rust_arg.ty));
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

                // APIVersion is Rust-specific and not mapped to C#
                lang::types::TypePattern::APIVersion => {
                    // Skip this pattern, it's not represented in C#
                    continue;
                }
            };

            id_map.set_ty(*rust_id, cs_id);
            kinds.set_kind(cs_id, TypeKind::TypePattern(cs_pattern));
            outcome.changed();
        }

        Ok(outcome)
    }
}
