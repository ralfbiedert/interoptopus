use crate::lang::types::TypeId;
use interoptopus::lang::meta::Docs;

pub struct Variant {
    pub name: String,
    pub docs: Docs,
    pub tag: usize,
    pub ty: TypeId,
}

pub struct DataEnum {
    pub variants: Vec<Variant>,
}
