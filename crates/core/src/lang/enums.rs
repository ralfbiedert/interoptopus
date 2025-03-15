use crate::lang::composite::Representation;
use crate::lang::{Composite, Documentation, Meta, Type, TypeInfo};

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
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VariantKind {
    Unit(usize),
    Typed(Box<Type>),
}

/// Variant and value of a [`Enum`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Variant {
    name: String,
    kind: VariantKind,
    documentation: Documentation,
}

impl Variant {
    #[must_use]
    pub const fn new(name: String, kind: VariantKind, documentation: Documentation) -> Self {
        Self { name, kind, documentation }
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
    pub const fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}
