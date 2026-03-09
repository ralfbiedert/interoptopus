use crate::lang::function::Signature;
use crate::lang::types::TypeId;

/// A pattern on a type level.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(clippy::large_enum_variant)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Rust's `c_void` type, which is not the same as `()` in return positions.
    CVoid,
    NamedCallback(Signature),
    AsyncCallback(TypeId),
    Vec(TypeId),
}
