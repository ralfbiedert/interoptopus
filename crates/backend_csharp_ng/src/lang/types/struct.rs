use crate::lang::types::{NamespaceId, TypeIdCs};

pub struct Field {
    name: String,
    vis: Visibility,
    ty: TypeIdCs,
}

pub struct Struct {
    fields: Vec<Field>,
}
