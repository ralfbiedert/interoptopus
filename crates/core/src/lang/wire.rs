//! A protobuf-like marshaller across the rust-ffi border.<sup>🚧</sup>
//! `Wire<T>` helpers to de-/serialize built-in types. <sup>:🚧</sup>
//
// ✅ String -> serialize as Vec<u8> but maybe Vec<u16> - see which is faster
// ✅ Vec<T> - usize len + this many T's
// ✅ HashMap<T,U> - usize len + this many (T,U)'s
// ✅ (), (T,...)
// ✅ Option<T> - bool + maybe T
// ✅ bool - 1u8 or 0u8
// ✅ arbitrary Structs - all fields in order of declaration
//
// Additionally, support serializing externally provided buffer (hopefully from C#).
//
// Generate serialization code on both sides, Rust and backend's language, to transfer
// type T over the FFI border in a byte array package.

use crate::lang::{Primitive, Type, WirePayload};
use std::collections::HashMap;

/// Used by the wire infrastructure to provide type information.
pub trait WireInfo {
    /// Provide a rust-side name of the type. Backends can convert it to their corresponding types.
    fn name() -> &'static str;
    /// Is this type layout fixed, or has variable elements. Useful for optimizing collection size calculations.
    fn is_fixed_size_element() -> bool;
    /// Provide a Type description, this is used to collect type hierarchy of wire types.
    fn wire_info() -> Type;
}

impl WireInfo for bool {
    fn name() -> &'static str {
        "bool"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::Bool)
    }
}

impl WireInfo for i8 {
    fn name() -> &'static str {
        "i8"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::I8)
    }
}
impl WireInfo for i16 {
    fn name() -> &'static str {
        "i16"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::I16)
    }
}
impl WireInfo for i32 {
    fn name() -> &'static str {
        "i32"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::I32)
    }
}
impl WireInfo for i64 {
    fn name() -> &'static str {
        "i64"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::I64)
    }
}

impl WireInfo for u8 {
    fn name() -> &'static str {
        "u8"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::U8)
    }
}
impl WireInfo for u16 {
    fn name() -> &'static str {
        "u16"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::U16)
    }
}
impl WireInfo for u32 {
    fn name() -> &'static str {
        "u32"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::U32)
    }
}
impl WireInfo for u64 {
    fn name() -> &'static str {
        "u64"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::U64)
    }
}

impl<T> WireInfo for Vec<T>
where
    T: WireInfo,
{
    fn name() -> &'static str {
        "Vec<T>" // @todo
    }
    fn is_fixed_size_element() -> bool {
        false
    }
    fn wire_info() -> Type {
        Type::WirePayload(WirePayload::Vec(Box::new(T::wire_info())))
    }
}

impl<T> WireInfo for Option<T>
where
    T: WireInfo,
{
    fn name() -> &'static str {
        "Option<T>" // @todo
    }
    fn is_fixed_size_element() -> bool {
        false
    }
    fn wire_info() -> Type {
        Type::WirePayload(WirePayload::Option(Box::new(T::wire_info())))
    }
}

impl<T, U, S> WireInfo for HashMap<T, U, S>
where
    T: WireInfo,
    U: WireInfo,
{
    fn name() -> &'static str {
        "HashMap<T,U>" // @todo
    }
    fn is_fixed_size_element() -> bool {
        false
    }
    fn wire_info() -> Type {
        Type::WirePayload(WirePayload::Map(Box::new(T::wire_info()), Box::new(U::wire_info())))
    }
}

impl WireInfo for String {
    fn name() -> &'static str {
        "String"
    }
    fn is_fixed_size_element() -> bool {
        false
    }
    fn wire_info() -> Type {
        Type::WirePayload(WirePayload::String)
    }
}
