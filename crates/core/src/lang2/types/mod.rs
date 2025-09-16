mod primitives;

use crate::lang2::meta::{Docs, Namespace, Visibility};
use crate::lang2::types::primitives::Primitive;
use crate::new_id;

new_id!(TypeId);

pub struct Field {}

pub struct Variant {}

pub struct Struct {
    fields: Vec<Field>,
}

pub struct Enum {
    Variants: Vec<Variant>,
}

pub enum TypeKind {
    Primitive(Primitive),
    Struct(Struct),
}

pub struct Type {
    pub namespace: Namespace,
    pub docs: Docs,
    pub visibility: Visibility,
    pub rust_name: String,
    pub kind: TypeKind,
}
