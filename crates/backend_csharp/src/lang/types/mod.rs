pub mod csharp;
pub mod kind;
mod overload;

use crate::lang::types::kind::TypeKind;
pub use overload::{DelegateFamily, OverloadFamily, PointerFamily};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ManagedConversion {
    /// Primitive types that convert via language built-ins
    AsIs,
    /// Conversion via `To...` methods, indicating no ownership transfer.
    To,
    /// Conversion via `Into...` methods, indicating ownership transfer.
    Into,
}

pub struct Type {
    // TODO: Handle this separately and not as part of model?
    // pub namespace: NamespaceId,
    pub name: String,
    pub kind: TypeKind,
}
