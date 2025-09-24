use crate::inventory::{ConstantId, Inventory, TypeId};
use crate::lang::meta::{Docs, Emission, Visibility};
use crate::lang::types::PrimitiveValue;

pub trait ConstantInfo {
    fn id() -> ConstantId;
    fn constant() -> Constant;
    fn register(inventory: &mut Inventory);
}

/// The value of a constant.
#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum Value {
    Primitive(PrimitiveValue),
}

pub struct Constant {
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub emission: Emission,
    pub ty: TypeId,
    pub value: Value,
}

pub trait ConstantValue {
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
