use crate::lang::types::kind::DataEnum;
use crate::lang::types::TypeId;

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
