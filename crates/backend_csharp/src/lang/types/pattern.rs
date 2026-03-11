use crate::lang::types::{DataEnum, TypeId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypePattern {
    ApiVersion,
    CStrPointer,
    Utf8String,
    Slice(TypeId),
    SliceMut(TypeId),
    Vec(TypeId),
    Option(TypeId, DataEnum),
    Result(TypeId, TypeId, DataEnum),
    Bool,
    CChar,
    CVoid,
    AsyncCallback(TypeId),
}
