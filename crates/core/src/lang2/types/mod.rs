mod array;
mod enums;
mod primitive;
mod structs;

use crate::lang2::function::Signature;
use crate::lang2::meta::{Docs, Emission, Visibility};
use crate::new_id;
pub use array::Array;
pub use enums::{Enum, Variant};
pub use primitive::{Primitive, PrimitiveValue};
pub use structs::{Field, Struct};

new_id!(TypeId);

pub enum TypeKind {
    Array(Array),
    Primitive(Primitive),
    Struct(Struct),
    Enum(Enum),
    FnPointer(Signature),
    Opaque,
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
