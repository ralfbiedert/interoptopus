use crate::lang2::function::FunctionId;
use crate::lang2::types::TypeId;

/// Combines a receiver, constructor, destructor and multiple methods in one entity.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Service {
    ty: TypeId,
    ctors: Vec<FunctionId>,
    destructor: FunctionId,
    methods: Vec<FunctionId>,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Builtins {
    functions: Vec<FunctionId>,
}

/// A pattern on a library level, usually involving both methods and types.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum LibraryPattern {
    Service(Service),
    Builtins(Builtins),
}
