//! Type definitions for the FFI data model.
//!
//! Every Rust type that crosses the FFI boundary is described by a [`Type`] carrying
//! a [`TypeKind`] discriminant. The kind determines the structure: [`Primitive`],
//! [`Struct`], [`Enum`], [`Array`], function pointer, pointer, or a higher-level
//! [`TypePattern`] (options, slices, strings, …).

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
pub use std::{type_id_ptr, type_id_ptr_mut};
pub use structs::{Field, Struct};
pub use wire::{WireIO, WireOnly};

/// Implemented by every Rust type that can appear in an FFI signature.
///
/// The `#[ffi]` attribute generates this for annotated structs and enums.
/// Primitive types and built-in patterns have hand-written implementations.
pub trait TypeInfo {
    /// Whether this type can be used inside a [`Wire<T>`](crate::lang::types::WireOnly).
    const WIRE_SAFE: bool;
    /// Whether this type can be passed directly over the FFI boundary.
    const RAW_SAFE: bool;
    /// Whether this type can appear in an async service method.
    const ASYNC_SAFE: bool;
    /// Whether this type can appear in a service method.
    const SERVICE_SAFE: bool;
    /// Whether this type is valid as a service constructor return type.
    const SERVICE_CTOR_SAFE: bool;

    /// The unique identifier for this type.
    fn id() -> TypeId;
    /// The structural kind of this type.
    fn kind() -> TypeKind;
    /// The full type description.
    fn ty() -> Type;

    /// Registers this type and all its transitive dependencies with the inventory.
    fn register(inventory: &mut impl Inventory);
}

/// The structural classification of an FFI type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeKind {
    /// A fixed-size array `[T; N]`.
    Array(Array),
    /// A primitive type (`u8`, `f32`, `bool`, …).
    Primitive(Primitive),
    /// A `#[repr(C)]` struct.
    Struct(Struct),
    /// A `#[repr(u32)]` (or similar) enum.
    Enum(Enum),
    /// A function pointer (`extern "C" fn(…) -> …`).
    FnPointer(Signature),
    /// A `*const T` pointer.
    ReadPointer(TypeId),
    /// A service (opaque, class-like).
    Service,
    /// A `*mut T` pointer.
    ReadWritePointer(TypeId),
    /// A type that may only be observed behind a pointer.
    Opaque,
    /// A type that can only appear inside a `Wire<T>`.
    WireOnly(WireOnly),
    /// A higher-level pattern type (option, slice, string, vec, …).
    TypePattern(TypePattern),
}

/// A named FFI type with its kind, documentation, and placement metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Type {
    /// The type name used in generated bindings.
    pub name: String,
    /// Whether the type is public or private.
    pub visibility: Visibility,
    /// Documentation extracted from `///` comments.
    pub docs: Docs,
    /// Where the type definition should be placed.
    pub emission: Emission,
    /// The structural kind.
    pub kind: TypeKind,
}

/// How a struct or enum is laid out in memory.
#[derive(Clone, Copy, Debug, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Layout {
    /// `#[repr(C)]` layout.
    C,
    /// `#[repr(transparent)]` layout.
    Transparent,
    /// `#[repr(C, packed)]` layout.
    Packed,
    /// Opaque (layout not exposed).
    Opaque,
    /// For use with enum discriminant (e.g., `#[repr(u32)]`).
    Primitive(Primitive),
}

/// The memory representation of a type: layout strategy plus optional alignment.
#[derive(Clone, Copy, Debug, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Repr {
    /// The layout strategy.
    pub layout: Layout,
    /// An explicit alignment override, if any.
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

/// Compile-time assertion that `T` is wire-safe.
#[track_caller]
pub const fn assert_wire_safe<T: TypeInfo>() {
    assert!(T::WIRE_SAFE);
}

/// Compile-time assertion that `T` can be passed directly over the FFI boundary.
#[track_caller]
pub const fn assert_raw_safe<T: TypeInfo>() {
    assert!(T::RAW_SAFE, "This type cannot be safely passed over FFI boundaries.");
}

/// Compile-time assertion that `T` can appear in an async service method.
#[track_caller]
pub const fn assert_async_safe<T: TypeInfo>() {
    assert!(T::ASYNC_SAFE);
}

/// Compile-time assertion that `T` can appear in a service method.
#[track_caller]
pub const fn assert_service_safe<T: TypeInfo>() {
    assert!(T::SERVICE_SAFE);
}

/// Compile-time assertion that `T` is a valid service constructor return type.
#[track_caller]
pub const fn assert_service_ctor_safe<T: TypeInfo>() {
    assert!(T::SERVICE_CTOR_SAFE, "This method looks like a constructor, but does not return ffi::Result<Self, _>");
}

/// Error returned when a wire-format serialization or deserialization fails.
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

impl From<::std::num::TryFromIntError> for SerializationError {
    fn from(e: ::std::num::TryFromIntError) -> Self {
        Self::Io(::std::io::Error::new(::std::io::ErrorKind::InvalidData, e))
    }
}
