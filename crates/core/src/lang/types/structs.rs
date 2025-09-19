use crate::lang::meta::{Docs, Visibility};
use crate::lang::types::{Repr, TypeId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub name: String,
    pub docs: Docs,
    pub visibility: Visibility,
    pub ty: TypeId,
}

impl Field {
    pub fn new(name: impl AsRef<str>, ty: TypeId) -> Self {
        Self { name: name.as_ref().to_string(), docs: Docs::default(), visibility: Visibility::Public, ty }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Struct {
    pub fields: Vec<Field>,
    pub repr: Repr,
}
