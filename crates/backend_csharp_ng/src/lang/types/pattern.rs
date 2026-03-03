use crate::lang::function::Signature;
use crate::lang::types::TypeId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypePattern {
    ApiVersion,
    CStrPointer,
    Utf8String,
    Slice(TypeId),
    SliceMut(TypeId),
    Vec(TypeId),
    Option(TypeId),
    Result(TypeId, TypeId),
    Bool,
    CChar,
    CVoid,
    NamedCallback(Signature),
    AsyncCallback(TypeId),
}
