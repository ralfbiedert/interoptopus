use crate::lang::TypeId;

/// A C# `Task` or `Task<T>` type used as the return type of async overloads.
#[derive(Debug, Clone)]
pub struct Task {
    /// The inner type for `Task<T>`, or `None` for bare `Task` (void result).
    pub inner: Option<TypeId>,
}
