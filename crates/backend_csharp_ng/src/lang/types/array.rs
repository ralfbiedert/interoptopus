use crate::model::TypeId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Array {
    pub ty: TypeId,
    pub len: usize,
}
