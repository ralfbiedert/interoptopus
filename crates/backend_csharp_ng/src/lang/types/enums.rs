use crate::lang::types::TypeId;

pub struct Variant {
    pub name: String,
    pub tag: usize,
    pub ty: TypeId,
}

pub struct DataEnum {
    pub variants: Vec<Variant>,
}
