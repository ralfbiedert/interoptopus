// TODO: How to deal with nested helper types?
//
// public partial struct EnumPayload
// {
//      [StructLayout(LayoutKind.Sequential)]
//      internal unsafe struct UnmanagedB { ... }
//
//      [StructLayout(LayoutKind.Sequential)]
//      internal unsafe struct UnmanagedC { ... }
//
//      [StructLayout(LayoutKind.Explicit)]
//      public unsafe struct Unmanaged { ... }
// }
//
// Q:
// - Are `Unmanaged*` here individual structs in our taxonomy?
// - Would they be linked nodes?
// - Should they be omitted entirely since they are an impl detail?
//
// A?
// - It appears if they are always "derived" from something (like an `Unmanaged` is always derived
//   from the actual type) they should not be listed anywhere, since it's genuinely an implementation
//   detail. But then again we might be hardcoding knowledge of whether an `Unmanaged` exists for
//   something into our code.
// - Instead, types should probably have an `ImplementationDetail` enum or fields or so, where it's
//   indicated for each type it its intended to be generated, and with that enum definitively
//   declaring what other parts of the code can expect to exist.

mod array;
mod composite;
pub mod csharp;
mod enums;
mod pattern;
mod pointer;
mod primitive;

use crate::lang::function::Signature;
use crate::model::TypeId;

pub use array::Array;
pub use composite::{Composite, CompositeKind, Field};
pub use enums::{DataEnum, Variant};
pub use pattern::TypePattern;
pub use pointer::Pointer;
pub use primitive::Primitive;

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

// TODO: Utopia
// 1) Any emitted bit that might be used by any other emitted bit must be "modelled"
// 2) Anything user visible should somehow be accessible via config options
// 3) Any pure implementation detail (not seen by other code here, or C# callers) doesn't matter
