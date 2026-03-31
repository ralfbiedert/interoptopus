#[doc(hidden)]
pub use interoptopus::lang::meta::{Emission, FileEmission, Module};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

impl Ord for Visibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.restrictiveness().cmp(&other.restrictiveness())
    }
}

impl PartialOrd for Visibility {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Visibility {
    /// Returns a restrictiveness rank where higher values mean more restricted.
    fn restrictiveness(&self) -> u8 {
        match self {
            Self::Public => 0,
            Self::Internal => 1,
            Self::Protected => 2,
            Self::Private => 3,
        }
    }

    /// Returns whichever of `self` and `other` is more restrictive.
    pub fn most_restrictive(self, other: &Self) -> Self {
        if other.restrictiveness() > self.restrictiveness() { other.clone() } else { self }
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::Protected => write!(f, "protected"),
            Self::Internal => write!(f, "internal"),
        }
    }
}

impl From<interoptopus::lang::meta::Visibility> for Visibility {
    fn from(v: interoptopus::lang::meta::Visibility) -> Self {
        match v {
            interoptopus::lang::meta::Visibility::Public => Self::Public,
            interoptopus::lang::meta::Visibility::Private => Self::Private,
        }
    }
}
