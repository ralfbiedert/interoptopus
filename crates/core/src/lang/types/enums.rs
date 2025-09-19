use crate::inventory::TypeId;
use crate::lang::meta::Docs;
use crate::lang::types::Repr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VariantKind {
    Unit(usize),
    Tuple(TypeId),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub name: String,
    pub docs: Docs,
    pub kind: VariantKind,
}

impl Variant {
    pub fn new(name: impl AsRef<str>, kind: VariantKind) -> Self {
        Self { name: name.as_ref().to_string(), docs: Docs::default(), kind }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Enum {
    pub variants: Vec<Variant>,
    pub repr: Repr,
}
