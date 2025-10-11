use crate::lang::meta::NamespaceId;
use crate::lang::types::TypeIdCs;

pub struct Variant {
    name: String,
    tag: usize,
    ty: TypeIdCs,
}

pub struct DataEnum {
    variants: Vec<Variant>,
}
