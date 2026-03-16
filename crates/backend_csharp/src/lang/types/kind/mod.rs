use crate::lang::types::kind::task::Task;
use crate::lang::TypeId;

mod array;
mod composite;
mod delegate;
mod enums;
mod pattern;
mod pointer;
mod primitive;
pub mod task;
mod util;

pub use self::{
    array::Array,
    composite::{Composite, Field},
    delegate::{Delegate, DelegateKind},
    enums::{DataEnum, Variant},
    pattern::TypePattern,
    pointer::{IntPtrHint, Pointer, PointerKind},
    primitive::Primitive,
    util::Util,
};

#[derive(Debug, Clone)]
pub enum TypeKind {
    Array(Array),
    AsyncHelper(TypeId),
    Composite(Composite),
    DataEnum(DataEnum),
    Delegate(Delegate),
    Opaque, // Regular opaques, not a service
    Pointer(Pointer),
    Primitive(Primitive),
    Service,
    Task(Task), // C# `Task` or `Task<T>` return type for async overloads.
    TypePattern(TypePattern),
    Util(Util),          // A backend-specific utility type (e.g., `InteropException`, `Utf8String` helper class).
    WireHelper(TypeId), // TODO?
}
