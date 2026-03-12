use crate::lang::TypeId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Array {
    pub ty: TypeId,
    pub len: usize,
}
