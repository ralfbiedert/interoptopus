/// How a function's return value is transformed in a body overload.
#[derive(Clone, Debug)]
pub enum RvalTransform {
    /// Return value passes through unchanged.
    PassThrough,
}

/// How a single argument is transformed in a body overload.
#[derive(Clone, Debug)]
pub enum ArgTransform {
    /// Argument passes through unchanged.
    PassThrough,
    /// Argument is passed by `ref` instead of by value.
    Ref,
    /// Argument is a bare C# delegate that wraps into a delegate class.
    WrapDelegate,
}
