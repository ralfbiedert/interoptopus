use crate::lang::{FunctionId, TypeId};

/// How a function's return value is transformed in an overload.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RvalTransform {
    /// Return value passes through unchanged.
    PassThrough,
    /// Return value is an async Task derived from the given Result type.
    AsyncTask(TypeId),
}

/// How a single argument is transformed in an overload.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ArgTransform {
    /// Argument passes through unchanged.
    PassThrough,
    /// Argument is passed by `ref` instead of by value.
    Ref,
    /// Argument is a bare C# delegate that wraps into a delegate class.
    WrapDelegate,
}

/// Per-function overload transforms describing how each argument and the return
/// value differ from the original native signature.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FnTransforms {
    pub rval: RvalTransform,
    /// The args refer to the overloaded function. If an overloads is missing a parameter
    /// (e.g., for async overloads that omit the last callback), this list is shorter than
    /// the original list of params.
    pub args: Vec<ArgTransform>,
}

/// Distinguishes the kind of each function overload registered in `overload::all`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OverloadKind {
    /// Simple overload: `IntPtr` → ref. No function body needed.
    Simple,
    /// Body overload: delegate wrapping, ref args. Has a function body with
    /// try/finally for disposal.
    Body(FnTransforms),
    /// Async overload: removes the callback arg, returns Task<T>.
    /// The `FnTransforms` covers all remaining arg transforms (ref, delegate wrap)
    /// so that async overloads compose with body-style transforms.
    Async(FnTransforms),
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Overload {
    pub kind: OverloadKind,
    pub base: FunctionId,
}
