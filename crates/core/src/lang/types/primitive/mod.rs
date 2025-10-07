mod nonzero;
mod numbers;
mod special;

use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub enum Primitive {
    Void,
    Bool,
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum PrimitiveValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Isize(isize),
    F32(f32),
    F64(f64),
}

impl Hash for PrimitiveValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Bool(v) => v.hash(state),
            Self::U8(v) => v.hash(state),
            Self::U16(v) => v.hash(state),
            Self::U32(v) => v.hash(state),
            Self::U64(v) => v.hash(state),
            Self::Usize(v) => v.hash(state),
            Self::I8(v) => v.hash(state),
            Self::I16(v) => v.hash(state),
            Self::I32(v) => v.hash(state),
            Self::I64(v) => v.hash(state),
            Self::Isize(v) => v.hash(state),
            Self::F32(v) => v.to_bits().hash(state),
            Self::F64(v) => v.to_bits().hash(state),
        }
    }
}

macro_rules! impl_const_value_primitive {
    (
        $rust_type:ty,
        $x:path
    ) => {
        impl From<$rust_type> for $crate::lang::constant::Value {
            fn from(x: $rust_type) -> Self {
                Self::Primitive($x(x))
            }
        }
    };
}

impl_const_value_primitive!(u8, PrimitiveValue::U8);
impl_const_value_primitive!(u16, PrimitiveValue::U16);
impl_const_value_primitive!(u32, PrimitiveValue::U32);
impl_const_value_primitive!(u64, PrimitiveValue::U64);
impl_const_value_primitive!(usize, PrimitiveValue::Usize);
impl_const_value_primitive!(i8, PrimitiveValue::I8);
impl_const_value_primitive!(i16, PrimitiveValue::I16);
impl_const_value_primitive!(i32, PrimitiveValue::I32);
impl_const_value_primitive!(i64, PrimitiveValue::I64);
impl_const_value_primitive!(isize, PrimitiveValue::Isize);
impl_const_value_primitive!(f32, PrimitiveValue::F32);
impl_const_value_primitive!(f64, PrimitiveValue::F64);
impl_const_value_primitive!(bool, PrimitiveValue::Bool);
