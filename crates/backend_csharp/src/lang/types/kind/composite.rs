use crate::lang::meta::Visibility;
use crate::lang::TypeId;
use interoptopus::lang::meta::Docs;
use interoptopus::lang::types::Repr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub name: String,
    pub docs: Docs,
    pub visibility: Visibility,
    pub ty: TypeId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Composite {
    pub fields: Vec<Field>,
    pub repr: Repr,
}
