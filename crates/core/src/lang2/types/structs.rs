use crate::lang2::meta::{Docs, Visibility};
use crate::lang2::types::{Repr, TypeId};

pub struct Field {
    pub name: String,
    pub docs: Docs,
    pub visibility: Visibility,
    pub ty: TypeId,
}

impl Field {
    pub fn new(name: impl AsRef<str>, ty: TypeId) -> Self {
        Self { name: name.as_ref().to_string(), docs: Default::default(), visibility: Visibility::Public, ty }
    }
}

pub struct Struct {
    pub fields: Vec<Field>,
    pub repr: Repr,
}
