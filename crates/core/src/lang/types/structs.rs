use crate::lang::meta::{Docs, Visibility};
use crate::lang::types::{Repr, TypeId};

/// A single named field of an FFI struct.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// The field name.
    pub name: String,
    /// Documentation extracted from `///` comments.
    pub docs: Docs,
    /// Whether the field is public or private.
    pub visibility: Visibility,
    /// The field's type.
    pub ty: TypeId,
}

impl Field {
    pub fn new(name: impl AsRef<str>, ty: TypeId) -> Self {
        Self { name: name.as_ref().to_string(), docs: Docs::default(), visibility: Visibility::Public, ty }
    }
}

/// An FFI struct definition with its fields and memory representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Struct {
    /// The struct's fields.
    pub fields: Vec<Field>,
    /// The memory representation (e.g., `#[repr(C)]`).
    pub repr: Repr,
}
