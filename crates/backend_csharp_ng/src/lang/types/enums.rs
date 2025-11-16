use crate::lang::types::TypeId;
use interoptopus::lang::meta::Docs;

#[derive(Clone)]
pub struct Variant {
    pub name: String,
    pub docs: Docs,
    pub tag: usize,
    pub ty: Option<TypeId>,
}

#[derive(Clone)]
pub struct DataEnum {
    pub variants: Vec<Variant>,
}
