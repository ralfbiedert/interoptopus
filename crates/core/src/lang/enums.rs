use crate::lang::composite::Representation;
use crate::lang::{Documentation, Meta};

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

/// Variant and value of a [`Enum`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Variant {
    name: String,
    value: usize,
    documentation: Documentation,
}

impl Variant {
    #[must_use]
    pub const fn new(name: String, value: usize, documentation: Documentation) -> Self {
        Self { name, value, documentation }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn value(&self) -> usize {
        self.value
    }

    #[must_use]
    pub const fn documentation(&self) -> &Documentation {
        &self.documentation
    }
}
