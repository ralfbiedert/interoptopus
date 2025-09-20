mod array;
mod enums;
mod fnptr;
mod pattern;
mod primitive;
mod std;
mod structs;
mod wire;

use crate::lang::function::Signature;
use crate::lang::meta::{Docs, Emission, Visibility};

use crate::inventory::{Inventory, TypeId};
pub use array::Array;
pub use enums::{Enum, Variant, VariantKind};
pub use pattern::TypePattern;
pub use primitive::{Primitive, PrimitiveValue};
pub use structs::{Field, Struct};
pub use wire::WireOnly;

pub trait TypeInfo {
    const WIRE_SAFE: bool;
    const RAW_SAFE: bool;
    const ASYNC_SAFE: bool;

    fn id() -> TypeId;
    fn kind() -> TypeKind;
    fn ty() -> Type;

    fn register(inventory: &mut Inventory);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeKind {
    Array(Array),
    Primitive(Primitive),
    Struct(Struct),
    Enum(Enum),
    FnPointer(Signature),
    ReadPointer(TypeId),
    Service,
    ReadWritePointer(TypeId),
    /// A type that may only be observed behind a pointer.
    Opaque,
    /// A type that can only appear inside a `Wire<T>`
    WireOnly(WireOnly),
    TypePattern(TypePattern),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Type {
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub emission: Emission,
    pub kind: TypeKind,
}

/// How a struct or enum is laid out in memory.
#[derive(Clone, Copy, Debug, PartialOrd, Eq, PartialEq, Hash)]
pub enum Layout {
    C,
    Transparent,
    Packed,
    Opaque,
    /// For use with enum discriminant.
    Primitive(Primitive),
}

/// How a type is represented in memory.
#[derive(Clone, Copy, Debug, PartialOrd, Eq, PartialEq, Hash)]
pub struct Repr {
    pub layout: Layout,
    pub alignment: Option<usize>,
}

impl Repr {
    #[must_use]
    pub fn c() -> Self {
        Self { layout: Layout::C, alignment: None }
    }

    #[must_use]
    pub fn u32() -> Self {
        Self { layout: Layout::Primitive(Primitive::U32), alignment: None }
    }
}

pub const fn assert_wire_safe<T: TypeInfo>() {
    assert!(T::WIRE_SAFE);
}
pub const fn assert_raw_safe<T: TypeInfo>() {
    assert!(T::RAW_SAFE);
}
pub const fn assert_async_safe<T: TypeInfo>() {
    assert!(T::ASYNC_SAFE);
}
