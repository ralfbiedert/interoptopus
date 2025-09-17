use crate::lang2::meta::Docs;
use crate::lang2::types::Repr;
use std::any::TypeId;

pub enum VariantKind {
    Unit(usize),
    Tuple(TypeId),
}

pub struct Variant {
    name: String,
    docs: Docs,
    kind: VariantKind,
}

pub struct Enum {
    variants: Vec<Variant>,
    repr: Repr,
}
