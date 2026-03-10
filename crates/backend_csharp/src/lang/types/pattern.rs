use crate::lang::types::{DataEnum, Delegate, TypeId};

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
    NamedCallback(Delegate),
    AsyncCallback(TypeId),
}
