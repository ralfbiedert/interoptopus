//! Maps Rust type patterns to C# type patterns.

use crate::lang::function::Signature;
use crate::lang::types::csharp;
use crate::lang::types::{TypeKind, TypePattern};
use crate::model::TypeId;
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{ModelResult, model_id_maps, model_type_kinds};
use interoptopus::lang;
use interoptopus::lang::types::TypeInfo;
use std::ffi::c_char;

#[derive(Default)]
pub struct Config {}

pub struct Pass {}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, kinds: &mut model_type_kinds::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let rust_pattern = match &ty.kind {
                lang::types::TypeKind::TypePattern(pattern) => pattern,
                _ => continue,
            };

            // Check if we already processed this pattern
            if id_map.cs_from_rust(*rust_id).is_some() {
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
                lang::types::TypePattern::Slice(rust_ty) => {
                    let Some(cs_ty) = id_map.cs_from_rust(*rust_ty) else {
                        continue;
                    };
                    (TypeId::from_id(rust_id.id()), TypePattern::Slice(cs_ty))
                }
                lang::types::TypePattern::SliceMut(rust_ty) => {
                    let Some(cs_ty) = id_map.cs_from_rust(*rust_ty) else {
                        continue;
                    };
                    (TypeId::from_id(rust_id.id()), TypePattern::SliceMut(cs_ty))
                }
                lang::types::TypePattern::Vec(rust_ty) => {
                    let Some(cs_ty) = id_map.cs_from_rust(*rust_ty) else {
                        continue;
                    };
                    (TypeId::from_id(rust_id.id()), TypePattern::Vec(cs_ty))
                }
                lang::types::TypePattern::Option(rust_ty) => {
                    let Some(cs_ty) = id_map.cs_from_rust(*rust_ty) else {
                        continue;
                    };
                    (TypeId::from_id(rust_id.id()), TypePattern::Option(cs_ty))
                }
                lang::types::TypePattern::AsyncCallback(rust_ty) => {
                    let Some(cs_ty) = id_map.cs_from_rust(*rust_ty) else {
                        continue;
                    };
                    (TypeId::from_id(rust_id.id()), TypePattern::AsyncCallback(cs_ty))
                }

                // Result pattern with two type parameters
                lang::types::TypePattern::Result(rust_ok, rust_err) => {
                    let Some(cs_ok) = id_map.cs_from_rust(*rust_ok) else {
                        continue;
                    };
                    let Some(cs_err) = id_map.cs_from_rust(*rust_err) else {
                        continue;
                    };
                    (TypeId::from_id(rust_id.id()), TypePattern::Result(cs_ok, cs_err))
                }

                // NamedCallback with signature
                lang::types::TypePattern::NamedCallback(rust_sig) => {
                    // Convert return type
                    let Some(cs_rval) = id_map.cs_from_rust(rust_sig.rval) else {
                        continue;
                    };

                    // Convert all arguments
                    let mut cs_arguments = Vec::new();
                    let mut all_args_available = true;

                    for rust_arg in &rust_sig.arguments {
                        let Some(cs_arg_type) = id_map.cs_from_rust(rust_arg.ty) else {
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

            id_map.set_rust_to_cs(*rust_id, cs_id);
            kinds.set_kind(cs_id, TypeKind::TypePattern(cs_pattern));
            outcome.changed();
        }

        Ok(outcome)
    }
}
