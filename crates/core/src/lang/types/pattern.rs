use crate::lang::function::Signature;
use crate::lang::types::TypeId;

/// Higher-level type patterns that map to idiomatic constructs in target languages.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(clippy::large_enum_variant)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypePattern {
    /// A null-terminated `*const c_char` ASCII string (`ffi::CStrPtr`).
    CStrPointer,
    /// A UTF-8 string (`ffi::String`).
    Utf8String,
    /// An API version constant.
    APIVersion,
    /// An immutable slice (`ffi::Slice<T>`).
    Slice(TypeId),
    /// A mutable slice (`ffi::SliceMut<T>`).
    SliceMut(TypeId),
    /// An optional value (`ffi::Option<T>`).
    Option(TypeId),
    /// A result value (`ffi::Result<T, E>`).
    Result(TypeId, TypeId),
    /// An FFI-safe boolean (`ffi::Bool`).
    Bool,
    /// An FFI-safe `c_char` (`ffi::CChar`).
    CChar,
    /// Rust's `c_void` type, which is not the same as `()` in return positions.
    CVoid,
    /// A named callback / function pointer with a full signature.
    NamedCallback(Signature),
    /// An async completion callback.
    AsyncCallback(TypeId),
    /// A growable array (`ffi::Vec<T>`).
    Vec(TypeId),
    /// A wire-serialized value (`Wire<T>`).
    Wire(TypeId),
}
