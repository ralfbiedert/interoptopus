use crate::inventory::TypeId;
use crate::lang::meta::Docs;
use crate::lang::types::Repr;

/// The payload of an enum variant.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VariantKind {
    /// A unit variant with an explicit discriminant value.
    Unit(isize),
    /// A tuple variant carrying a single payload type.
    Tuple(TypeId),
}

/// A single variant of an FFI enum.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    /// The variant name.
    pub name: String,
    /// Documentation extracted from `///` comments.
    pub docs: Docs,
    /// The variant's payload.
    pub kind: VariantKind,
}

impl Variant {
    pub fn new(name: impl AsRef<str>, kind: VariantKind) -> Self {
        Self { name: name.as_ref().to_string(), docs: Docs::default(), kind }
    }
}

/// An FFI enum definition with its variants and memory representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enum {
    /// The enum's variants.
    pub variants: Vec<Variant>,
    /// The memory representation (e.g., `#[repr(u32)]`).
    pub repr: Repr,
}
