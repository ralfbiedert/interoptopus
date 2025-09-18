mod array;
mod enums;
mod fnptr;
mod pattern;
mod primitive;
mod std;
mod structs;
mod wire;

use crate::lang2::function::Signature;
use crate::lang2::meta::{Docs, Emission, Visibility};

use crate::inventory2::TypeId;
pub use array::Array;
pub use enums::{Enum, Variant};
pub use pattern::TypePattern;
pub use primitive::{Primitive, PrimitiveValue};
pub use structs::{Field, Struct};
pub use wire::WireOnly;

pub enum TypeKind {
    Array(Array),
    Primitive(Primitive),
    Struct(Struct),
    Enum(Enum),
    FnPointer(Signature),
    ReadPointer(TypeId),
    ReadWritePointer(TypeId),
    Opaque,
    /// A type that can only appear inside a `Wire<T>`
    WireOnly(WireOnly),
    TypePattern(TypePattern),
}

pub struct Type {
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub emission: Emission,
    pub kind: TypeKind,
}

/// How a struct or enum is laid out in memory.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Layout {
    C,
    Transparent,
    Packed,
    Opaque,
    /// For use with enum discriminant.
    Primitive(crate::lang::Primitive),
}

/// How a type is represented in memory.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Repr {
    layout: crate::lang::Layout,
    alignment: Option<usize>,
}
