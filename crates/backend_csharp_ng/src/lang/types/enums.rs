use crate::lang::types::TypeId;

pub struct Variant {
    name: String,
    tag: usize,
    ty: TypeId,
}

pub struct DataEnum {
    variants: Vec<Variant>,
}
