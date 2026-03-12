use crate::lang::types::TypeId;
use interoptopus::lang::meta::Docs;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub docs: Docs,
    pub tag: usize,
    pub ty: Option<TypeId>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DataEnum {
    pub variants: Vec<Variant>,
}
