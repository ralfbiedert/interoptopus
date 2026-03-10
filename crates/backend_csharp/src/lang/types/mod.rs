mod array;
mod composite;
pub mod csharp;
mod enums;
mod pattern;
mod pointer;
mod primitive;

use crate::lang::function::Signature;
use crate::lang::TypeId;

pub use array::Array;
pub use composite::{Composite, Field};
pub use enums::{DataEnum, Variant};
pub use pattern::TypePattern;
pub use pointer::{IntPtrHint, Pointer};
pub use primitive::Primitive;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ManagedConversion {
    /// Primitive types that convert via language built-ins
    AsIs,
    /// Conversion via `To...` methods, indicating no ownership transfer.
    To,
    /// Conversion via `Into...` methods, indicating ownership transfer.
    Into,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Primitive(Primitive),
    Array(Array),
    DataEnum(DataEnum),
    Composite(Composite),
    Delegate(Signature),
    Service,
    Opaque,           // Regular opaques, not a service
    Pointer(Pointer), // (can become `ref` in signatures, or `IntPtr` in sigs or fields).
    AsyncHelper(TypeId),
    WireHelper(TypeId), // TODO?
    TypePattern(TypePattern),
}

pub struct Type {
    // TODO: Handle this separately and not as part of model?
    // pub namespace: NamespaceId,
    pub name: String,
    pub kind: TypeKind,
}
