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
use ::std::io::{Read, Write};

use crate::inventory::{Inventory, TypeId};
pub use array::Array;
pub use enums::{Enum, Variant, VariantKind};
pub use pattern::TypePattern;
pub use primitive::{Primitive, PrimitiveValue};
pub use structs::{Field, Struct};
pub use wire::WireOnly;

pub trait TypeInfo {
    // Flags that govern where a type can be used. Allows
    // us to emit `const {}` guard blocks to produce compile time
    // errors.
    const WIRE_SAFE: bool;
    const RAW_SAFE: bool;
    const ASYNC_SAFE: bool;
    const SERVICE_SAFE: bool;
    const SERVICE_CTOR_SAFE: bool;

    // Basic helpers producing infos about the type
    fn id() -> TypeId;
    fn kind() -> TypeKind;
    fn ty() -> Type;

    // Registers the type and all dependents in an Inventory.
    fn register(inventory: &mut Inventory);

    // Utilities for (de)serializing an instance of this type. These must be
    // properly implemented iff WIRE_SAFE is true. Otherwise these
    // should panic.
    fn write(&self, out: &mut impl Write) -> Result<(), SerializationError>;
    fn read(input: &mut impl Read) -> Result<Self, SerializationError>;
    fn live_size(&self) -> usize;
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

#[track_caller]
pub const fn assert_wire_safe<T: TypeInfo>() {
    assert!(T::WIRE_SAFE);
}

#[track_caller]
pub const fn assert_raw_safe<T: TypeInfo>() {
    assert!(T::RAW_SAFE, "This type cannot be safely passed over FFI boundaries.");
}

#[track_caller]
pub const fn assert_async_safe<T: TypeInfo>() {
    assert!(T::ASYNC_SAFE);
}

#[track_caller]
pub const fn assert_service_safe<T: TypeInfo>() {
    assert!(T::SERVICE_SAFE);
}

#[track_caller]
pub const fn assert_service_ctor_safe<T: TypeInfo>() {
    assert!(T::SERVICE_CTOR_SAFE, "This method looks like a constructor, but does not return ffi::Result<Self, _>");
}

/// If a wire transfer goes wrong.
// @todo play with implementing it as a struct?
#[derive(Debug)]
pub enum SerializationError {
    Io(::std::io::Error),
    InvalidData(String),
    InvalidDiscriminant(String, usize),
}

impl From<::std::io::Error> for SerializationError {
    fn from(e: ::std::io::Error) -> Self {
        Self::Io(e)
    }
}
