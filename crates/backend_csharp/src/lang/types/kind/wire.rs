use crate::lang::TypeId;
use crate::lang::types::kind::Composite;

#[derive(Debug, Clone)]
pub enum WireOnly {
    /// For composite types that contain wire-only somewhere in their hierarchy.
    /// These have no regular composite machinery, but are just C# data types.
    Composite(Composite),
    String,
    Vec(TypeId),
    /// Some true `std::option::Option<T>` that became a nullable in C#
    Nullable(TypeId),
    Map(TypeId, TypeId),
}
