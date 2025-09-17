use crate::lang2::meta::{Docs, Visibility};
use crate::lang2::types::{Repr, TypeId};

pub struct Field {
    name: String,
    docs: Docs,
    visibility: Visibility,
    ty: TypeId,
}

pub struct Struct {
    fields: Vec<Field>,
    repr: Repr,
}
