#[doc(hidden)]
pub use interoptopus::lang::meta::{Emission, FileEmission, Module};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}
