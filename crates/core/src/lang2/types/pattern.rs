use crate::lang2::function::{Argument, Signature};
use crate::lang2::types::enums::VariantKind;
use crate::lang2::types::fnptr::fnptr_typeid;
use crate::lang2::types::std::{ptr_mut_typeid, ptr_typeid};
use crate::lang2::types::{Enum, Field, Primitive, Repr, Struct, TypeId, TypeInfo, TypeKind, Variant};
use std::ffi::{c_char, c_void};

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
        TypePattern::APIVersion => TypeKind::Primitive(Primitive::U64),
        TypePattern::Slice(t) => TypeKind::Struct(Struct { fields: vec![Field::new("ptr", ptr_typeid(*t)), Field::new("len", u64::id())], repr: Repr::c() }),
        TypePattern::SliceMut(t) => TypeKind::Struct(Struct { fields: vec![Field::new("ptr", ptr_mut_typeid(*t)), Field::new("len", u64::id())], repr: Repr::c() }),
        TypePattern::Option(t) => {
            TypeKind::Enum(Enum { variants: vec![Variant::new("Some", VariantKind::Tuple(*t)), Variant::new("None", VariantKind::Unit(1))], repr: Repr::u32() })
        }
        TypePattern::Result(t, e) => TypeKind::Enum(Enum {
            variants: vec![
                Variant::new("Ok", VariantKind::Tuple(*t)),
                Variant::new("Err", VariantKind::Tuple(*e)),
                Variant::new("Panic", VariantKind::Unit(2)),
                Variant::new("Null", VariantKind::Unit(3)),
            ],
            repr: Repr::u32(),
        }),
        TypePattern::Bool => TypeKind::Primitive(Primitive::Bool),
        TypePattern::CChar => TypeKind::Primitive(Primitive::I8),
        TypePattern::CVoid => TypeKind::Primitive(Primitive::Void),
        TypePattern::NamedCallback(x) => {
            let mut sig_with_voidptr = x.clone();
            sig_with_voidptr.arguments.push(Argument::new("data", <*const c_void>::id()));

            TypeKind::Struct(Struct { fields: vec![Field::new("fnptr", fnptr_typeid(&sig_with_voidptr)), Field::new("data", <*const c_void>::id())], repr: Repr::c() })
        }
        TypePattern::AsyncCallback(x) => {
            let sig = Signature { arguments: vec![Argument::new("ref", ptr_typeid(*x)), Argument::new("data", <*const c_void>::id())], rval: <()>::id() };
            TypeKind::Struct(Struct { fields: vec![Field::new("fnptr", fnptr_typeid(&sig)), Field::new("data", <*const c_void>::id())], repr: Repr::c() })
        }
        TypePattern::Vec(t) => TypeKind::Struct(Struct {
            fields: vec![Field::new("ptr", ptr_mut_typeid(*t)), Field::new("len", u64::id()), Field::new("capacity", u64::id())],
            repr: Repr::c(),
        }),
    }
}
