//! Computes the C-level fallback TypeKind for each pattern type.
//!
//! For each Rust TypePattern, this stores the equivalent C-level TypeKind
//! (the "unrolled" representation). Struct-based patterns like Slice become
//! Composite with ptr/len fields; enum-based patterns like Option/Result
//! become DataEnum with their variants.
//!
//! All inner type references are resolved through the id_map from Rust TypeIds
//! to C# TypeIds, relying on the convergence loop to retry when dependencies
//! aren't mapped yet.

use crate::lang::meta::Visibility;
use crate::lang::types::kind::{Composite, DataEnum, Field, IntPtrHint, Pointer, PointerKind, Primitive, TypeKind, Variant};
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::try_extract_kind;
use interoptopus::lang;
use interoptopus::lang::meta::Docs;
use interoptopus::lang::types::{type_id_ptr, type_id_ptr_mut, Repr, TypeInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    fallbacks: HashMap<TypeId, TypeKind>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, fallbacks: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, id_map: &model::id_map::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let mut outcome = Unchanged;

        // Static Rust TypeIds for commonly needed types and pointers.

        for (rust_id, ty) in rs_types {
            let rust_pattern = try_extract_kind!(ty, TypePattern);
            let Some(cs_id) = id_map.ty(*rust_id) else { continue };

            if self.fallbacks.contains_key(&cs_id) {
                continue;
            }

            let fallback = match rust_pattern {
                lang::types::TypePattern::CStrPointer => {
                    // *const c_char
                    let Some(cs_ptr) = id_map.ty(<*const std::ffi::c_char>::id()) else { continue };
                    TypeKind::Pointer(Pointer { kind: PointerKind::IntPtr(IntPtrHint::Read), target: cs_ptr })
                }
                lang::types::TypePattern::Utf8String => {
                    // { *mut u8, u64, u64 }
                    let Some(cs_ptr) = id_map.ty(<*mut u8>::id()) else { continue };
                    let Some(cs_u64) = id_map.ty(u64::id()) else { continue };
                    TypeKind::Composite(Composite { fields: vec![field("ptr", cs_ptr), field("len", cs_u64), field("capacity", cs_u64)], repr: Repr::c() })
                }
                lang::types::TypePattern::APIVersion => TypeKind::Primitive(Primitive::ULong),
                lang::types::TypePattern::Slice(rust_ty) => {
                    // { *const T, u64 }
                    let Some(cs_ptr) = id_map.ty(type_id_ptr(*rust_ty)) else { continue };
                    let Some(cs_u64) = id_map.ty(u64::id()) else { continue };
                    TypeKind::Composite(Composite { fields: vec![field("ptr", cs_ptr), field("len", cs_u64)], repr: Repr::c() })
                }
                lang::types::TypePattern::SliceMut(rust_ty) => {
                    // { *mut T, u64 }
                    let Some(cs_ptr) = id_map.ty(type_id_ptr_mut(*rust_ty)) else { continue };
                    let Some(cs_u64) = id_map.ty(u64::id()) else { continue };
                    TypeKind::Composite(Composite { fields: vec![field("ptr", cs_ptr), field("len", cs_u64)], repr: Repr::c() })
                }
                lang::types::TypePattern::Vec(rust_ty) => {
                    // { *mut T, u64, u64 }
                    let Some(cs_ptr) = id_map.ty(type_id_ptr_mut(*rust_ty)) else { continue };
                    let Some(cs_u64) = id_map.ty(u64::id()) else { continue };
                    TypeKind::Composite(Composite { fields: vec![field("ptr", cs_ptr), field("len", cs_u64), field("capacity", cs_u64)], repr: Repr::c() })
                }
                lang::types::TypePattern::Option(rust_ty) => {
                    let Some(payload) = resolve_payload(*rust_ty, id_map) else { continue };
                    TypeKind::DataEnum(DataEnum { variants: vec![variant("Some", 0, payload), variant("None", 1, None)] })
                }
                lang::types::TypePattern::Result(rust_ok, rust_err) => {
                    let Some(ok_payload) = resolve_payload(*rust_ok, id_map) else { continue };
                    let Some(err_payload) = resolve_payload(*rust_err, id_map) else { continue };
                    TypeKind::DataEnum(DataEnum {
                        variants: vec![
                            variant("Ok", 0, ok_payload),
                            variant("Err", 1, err_payload),
                            variant("Panic", 2, None),
                            variant("Null", 3, None),
                        ],
                    })
                }
                lang::types::TypePattern::Bool => TypeKind::Primitive(Primitive::Byte),
                lang::types::TypePattern::CChar => TypeKind::Primitive(Primitive::SByte),
                lang::types::TypePattern::CVoid => TypeKind::Primitive(Primitive::Void),
                lang::types::TypePattern::NamedCallback(_) | lang::types::TypePattern::AsyncCallback(_) => {
                    // { *mut c_void, *mut c_void }
                    let Some(cs_void_ptr) = id_map.ty(<*mut std::ffi::c_void>::id()) else { continue };
                    TypeKind::Composite(Composite { fields: vec![field("fnptr", cs_void_ptr), field("data", cs_void_ptr)], repr: Repr::c() })
                }
            };

            self.fallbacks.insert(cs_id, fallback);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn get(&self, id: TypeId) -> Option<&TypeKind> {
        self.fallbacks.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &TypeKind)> {
        self.fallbacks.iter()
    }
}

fn field(name: &str, ty: TypeId) -> Field {
    Field { name: name.to_string(), docs: Docs::default(), visibility: Visibility::Public, ty }
}

fn variant(name: &str, tag: usize, ty: Option<TypeId>) -> Variant {
    Variant { name: name.to_string(), docs: Docs::default(), tag, ty }
}

/// Resolves a Rust type to an optional C# variant payload.
/// Void types (`()`) become `Some(None)` (no payload), non-void types become
/// `Some(Some(cs_id))`, and not-yet-mapped types return `None`.
fn resolve_payload(rust_ty: interoptopus::inventory::TypeId, id_map: &model::id_map::Pass) -> Option<Option<TypeId>> {
    if rust_ty == <()>::id() { Some(None) } else { id_map.ty(rust_ty).map(Some) }
}
