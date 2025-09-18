use crate::inventory2::Inventory;
use crate::lang2::function::Signature;
use crate::lang2::meta::{Emission, Visibility};
use crate::lang2::types::{Field, Repr, Struct, Type, TypeId, TypeInfo, TypeKind};
use std::ffi::c_char;
use std::process::id;
use std::u64;

/// A pattern on a type level.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum TypePattern {
    CStrPointer,
    Utf8String,
    APIVersion,
    Slice(TypeId),
    SliceMut(TypeId),
    Option(TypeId),
    Result(TypeId, TypeId),
    Bool,
    CChar,
    /// Rust's `c_void` type, which is not the same as `()` in return positions.
    CVoid,
    NamedCallback(Signature),
    AsyncCallback(TypeId),
    Vec(TypeId),
}

pub fn fallback_type(pattern: &TypePattern) -> TypeKind {
    match pattern {
        TypePattern::CStrPointer => TypeKind::ReadPointer(c_char::id()),
        TypePattern::Utf8String => TypeKind::Struct(Struct {
            fields: vec![Field::new("ptr", <*mut u8>::id()), Field::new("len", u64::id()), Field::new("capacity", u64::id())],
            repr: Repr::c(),
        }),
        // TODO next
        TypePattern::APIVersion => {}
        TypePattern::Slice(_) => {}
        TypePattern::SliceMut(_) => {}
        TypePattern::Option(_) => {}
        TypePattern::Result(t, e) => {}
        TypePattern::Bool => {}
        TypePattern::CChar => {}
        TypePattern::CVoid => {}
        TypePattern::NamedCallback(_) => {}
        TypePattern::AsyncCallback(_) => {}
        TypePattern::Vec(_) => {}
    }
}
