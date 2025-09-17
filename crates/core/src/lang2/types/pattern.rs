use crate::lang2::function::Signature;
use crate::lang2::types::TypeId;

/// A pattern on a type level.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum TypePattern {
    CStrPointer,
    Utf8String,
    APIVersion,
    Slice(TypeId),
    SliceMut(TypeId),
    Option(TypeId),
    Result(TypeId, TypeId),
    Bool,
    CChar,
    NamedCallback(Signature),
    AsyncCallback(TypeId),
    Vec(TypeId),
}
