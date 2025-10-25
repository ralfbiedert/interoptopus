use crate::lang::types::TypeId;

pub enum TypePattern {
    Utf8String,
    Slice(TypeId),
    SliceMut(TypeId),
    Vec(TypeId),
    Option(TypeId),
    Result(TypeId, TypeId),
}
