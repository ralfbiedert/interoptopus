use crate::lang::TypeId;
use crate::lang::types::kind::Primitive;
use interoptopus::lang::meta::Docs;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub docs: Docs,
    pub tag: isize,
    pub ty: Option<TypeId>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DataEnum {
    pub variants: Vec<Variant>,
    /// The C# primitive used for the discriminant field.
    pub discriminant_type: Primitive,
}
