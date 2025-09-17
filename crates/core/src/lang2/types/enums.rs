use crate::lang2::meta::Docs;
use crate::lang2::types::Repr;
use std::any::TypeId;

pub enum VariantKind {
    Unit(usize),
    Tuple(TypeId),
}

pub struct Variant {
    pub name: String,
    pub docs: Docs,
    pub kind: VariantKind,
}

pub struct Enum {
    pub variants: Vec<Variant>,
    pub repr: Repr,
}
