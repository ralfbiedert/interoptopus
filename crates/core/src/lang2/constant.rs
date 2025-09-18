use crate::inventory2::TypeId;
use crate::lang2::meta::{Docs, Emission, Visibility};
use crate::lang2::types::PrimitiveValue;

/// The value of a constant.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum ConstantValue {
    Primitive(PrimitiveValue),
}

pub struct Constant {
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub emission: Emission,
    pub ty: TypeId,
    pub value: ConstantValue,
}
