use crate::lang::composite::Representation;
use crate::lang::{Docs, Meta, Type};

/// A (C-style) `enum` containing numbered variants.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Enum {
    name: String,
    variants: Vec<Variant>,
    repr: Representation,
    meta: Meta,
}

impl Enum {
    #[must_use]
    pub const fn new(name: String, variants: Vec<Variant>, meta: Meta, repr: Representation) -> Self {
        Self { name, variants, repr, meta }
    }

    #[must_use]
    pub fn rust_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn variants(&self) -> &[Variant] {
        &self.variants
    }

    #[must_use]
    pub fn variant_by_name(&self, name: &str) -> Option<Variant> {
        self.variants.iter().find(|x| x.name == name).cloned()
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    #[must_use]
    pub const fn repr(&self) -> &Representation {
        &self.repr
    }

    #[must_use]
    pub fn to_type(&self) -> Type {
        Type::Enum(self.clone())
    }
}

/// If this is a unit variant `E::A` or typed variant `E::B(T)`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VariantKind {
    Unit(usize),
    Typed(usize, Box<Type>),
}

impl VariantKind {
    #[must_use]
    pub const fn is_unit(&self) -> bool {
        matches!(self, Self::Unit(_))
    }

    #[must_use]
    pub const fn is_typed(&self) -> bool {
        matches!(self, Self::Typed(_, _))
    }

    #[must_use]
    pub const fn as_unit(&self) -> Option<usize> {
        if let Self::Unit(x) = self { Some(*x) } else { None }
    }

    #[must_use]
    pub const fn as_typed(&self) -> Option<&Type> {
        if let Self::Typed(_, x) = self { Some(x) } else { None }
    }
}

/// Variant and value of a [`Enum`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Variant {
    name: String,
    kind: VariantKind,
    docs: Docs,
}

impl Variant {
    #[must_use]
    pub const fn new(name: String, kind: VariantKind, docs: Docs) -> Self {
        Self { name, kind, docs }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn kind(&self) -> &VariantKind {
        &self.kind
    }

    #[must_use]
    pub const fn docs(&self) -> &Docs {
        &self.docs
    }
}
