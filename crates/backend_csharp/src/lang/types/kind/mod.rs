use crate::lang::TypeId;

mod array;
mod composite;
mod delegate;
mod enums;
mod pattern;
mod pointer;
mod primitive;

pub use self::{
    array::Array,
    composite::{Composite, Field},
    delegate::{Delegate, DelegateKind},
    enums::{DataEnum, Variant},
    pattern::TypePattern,
    pointer::{IntPtrHint, Pointer, PointerKind},
    primitive::Primitive,
};

#[derive(Debug, Clone)]
pub enum TypeKind {
    Primitive(Primitive),
    Array(Array),
    DataEnum(DataEnum),
    Composite(Composite),
    Delegate(Delegate),
    Service,
    Opaque,           // Regular opaques, not a service
    Pointer(Pointer), // (can become `ref` in signatures, or `IntPtr` in sigs or fields).
    AsyncHelper(TypeId),
    WireHelper(TypeId), // TODO?
    TypePattern(TypePattern),
}
