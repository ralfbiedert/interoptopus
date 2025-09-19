use crate::lang::types::TypeId;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WireOnly {
    String,
    Vec(TypeId),
    Map(TypeId, TypeId),
}
