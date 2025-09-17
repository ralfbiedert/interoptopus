use crate::lang2::meta::{Docs, Visibility};
use crate::lang2::types::{Repr, TypeId};

pub struct Field {
    pub name: String,
    pub docs: Docs,
    pub visibility: Visibility,
    pub ty: TypeId,
}

pub struct Struct {
    pub fields: Vec<Field>,
    pub repr: Repr,
}
