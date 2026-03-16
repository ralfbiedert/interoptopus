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
    /// C# `Task` or `Task<T>` return type for async overloads.
    Task(Task),
    AsyncHelper(TypeId),
    WireHelper(TypeId), // TODO?
    TypePattern(TypePattern),
    /// A backend-specific utility type (e.g., `InteropException`, `Utf8String` helper class).
    Util,
}

/// A C# `Task` or `Task<T>` type used as the return type of async overloads.
#[derive(Debug, Clone)]
pub struct Task {
    /// The inner type for `Task<T>`, or `None` for bare `Task` (void result).
    pub inner: Option<TypeId>,
}
