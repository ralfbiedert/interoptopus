use crate::lang::Meta;

/// An included type only known by name.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Included {
    name: String,
    meta: Meta,
}

impl Included {
    /// Create a new included type.
    #[must_use]
    pub const fn new(name: String, meta: Meta) -> Self {
        Self { name, meta }
    }

    /// Get the name of the included type.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the meta information of the included type.
    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }
}
