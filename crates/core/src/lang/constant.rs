//! FFI constants and their values.

use crate::inventory::{ConstantId, Inventory, TypeId};
use crate::lang::meta::{Docs, Emission, Visibility};
use crate::lang::types::PrimitiveValue;

/// Implemented by companion types generated for `#[ffi]` constants.
///
/// You do not implement this manually — the `#[ffi]` attribute on a `const` item
/// generates a zero-sized struct that implements this trait.
pub trait ConstantInfo {
    /// The unique identifier for this constant.
    fn id() -> ConstantId;
    /// Returns the full constant description.
    fn constant() -> Constant;
    /// Registers this constant with the given inventory.
    fn register(inventory: &mut impl Inventory);
}

/// The value of a constant.
#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    /// A primitive value (integer, float, or bool).
    Primitive(PrimitiveValue),
}

/// A named constant exported across the FFI boundary.
#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Constant {
    /// The name used in generated bindings.
    pub name: String,
    /// Whether the constant is public or private.
    pub visibility: Visibility,
    /// Documentation extracted from `///` comments.
    pub docs: Docs,
    /// Where the constant definition should be placed.
    pub emission: Emission,
    /// The type of the constant's value.
    pub ty: TypeId,
    /// The constant's value.
    pub value: Value,
}

/// Trait for Rust primitive types that can be used as constant values.
pub trait ConstantValue {
    /// Wraps `self` in a [`Value`].
    fn value(&self) -> Value;
}

macro_rules! constant_value {
    ($ty:ty, $x:ident) => {
        impl ConstantValue for $ty {
            fn value(&self) -> Value {
                Value::Primitive(PrimitiveValue::$x(*self))
            }
        }
    };
}

constant_value!(bool, Bool);
constant_value!(u8, U8);
constant_value!(u16, U16);
constant_value!(u32, U32);
constant_value!(u64, U64);
constant_value!(usize, Usize);
constant_value!(i8, I8);
constant_value!(i16, I16);
constant_value!(i32, I32);
constant_value!(i64, I64);
constant_value!(isize, Isize);
constant_value!(f32, F32);
constant_value!(f64, F64);
