use crate::lang::{Meta, Primitive, PrimitiveValue, Type};
use std::hash::{Hash, Hasher};

/// The value of a constant.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum ConstantValue {
    Primitive(PrimitiveValue),
}

impl ConstantValue {
    pub(crate) fn fucking_hash_it_already<H: Hasher>(&self, h: &mut H) {
        match self {
            Self::Primitive(x) => match x {
                PrimitiveValue::Bool(x) => x.hash(h),
                PrimitiveValue::U8(x) => x.hash(h),
                PrimitiveValue::U16(x) => x.hash(h),
                PrimitiveValue::U32(x) => x.hash(h),
                PrimitiveValue::U64(x) => x.hash(h),
                PrimitiveValue::I8(x) => x.hash(h),
                PrimitiveValue::I16(x) => x.hash(h),
                PrimitiveValue::I32(x) => x.hash(h),
                PrimitiveValue::I64(x) => x.hash(h),
                PrimitiveValue::F32(x) => x.to_le_bytes().hash(h),
                PrimitiveValue::F64(x) => x.to_le_bytes().hash(h),
            },
        }
    }
}

/// A Rust `const` definition with a name and value, might become a `#define`.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Constant {
    name: String,
    value: ConstantValue,
    meta: Meta,
}

impl Constant {
    #[must_use]
    pub const fn new(name: String, value: ConstantValue, meta: Meta) -> Self {
        Self { name, value, meta }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn value(&self) -> &ConstantValue {
        &self.value
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    /// Returns the type of this constant.
    #[must_use]
    pub const fn the_type(&self) -> Type {
        match &self.value {
            ConstantValue::Primitive(x) => Type::Primitive(match x {
                PrimitiveValue::Bool(_) => Primitive::Bool,
                PrimitiveValue::U8(_) => Primitive::U8,
                PrimitiveValue::U16(_) => Primitive::U16,
                PrimitiveValue::U32(_) => Primitive::U32,
                PrimitiveValue::U64(_) => Primitive::U64,
                PrimitiveValue::I8(_) => Primitive::I8,
                PrimitiveValue::I16(_) => Primitive::I16,
                PrimitiveValue::I32(_) => Primitive::I32,
                PrimitiveValue::I64(_) => Primitive::I64,
                PrimitiveValue::F32(_) => Primitive::F32,
                PrimitiveValue::F64(_) => Primitive::F64,
            }),
        }
    }
}
