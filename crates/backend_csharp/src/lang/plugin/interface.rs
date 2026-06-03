use crate::lang::FunctionId;
use crate::lang::TypeId;
use crate::lang::functions::Signature;
use interoptopus::lang::meta::Emission;

#[derive(Debug, PartialEq, Eq)]
pub enum InterfaceKind {
    Plugin,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
    Regular,
    Static,
}

/// Classification of how a method's `Result<T, E>` return type should be handled
/// at the trampoline boundary.
///
/// The two variants are mutually exclusive by construction: a Result is either
/// peeled off the user-facing signature (Try) or kept (Passthrough), never both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultKind {
    /// `Result<T, ExceptionError>` was unwrapped from the user-facing signature so
    /// the plugin method returns just `T`. Trampoline wraps the call with
    /// `FromCall` to map typed C# exceptions back to `Err` variants. Holds the
    /// original Result `TypeId`.
    Try(TypeId),
    /// User-defined `Result<T, E>` was kept on the user-facing signature; the
    /// plugin method returns the full Result. Trampoline wraps the call with
    /// `FromCallResult` so that any uncaught C# exception is folded into the
    /// `Panic` variant on the wire. Holds the Result `TypeId`.
    Passthrough(TypeId),
}

impl ResultKind {
    /// The Result's C# `TypeId`, regardless of variant.
    #[must_use]
    pub fn type_id(self) -> TypeId {
        match self {
            Self::Try(id) | Self::Passthrough(id) => id,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Method {
    /// C# method name (e.g. `"PrimitiveU32"`, `"ServiceaCreate"`).
    pub name: String,
    /// Whether this is a static or instance method.
    pub kind: MethodKind,
    /// The original FFI function this method maps to.
    pub base: FunctionId,
    /// C#-ified signature (async callback stripped, args resolved).
    pub csharp: Signature,
    /// Resolved C# return type (e.g. the sibling `ResultTypeId`, service `TypeId`, etc.).
    pub rval_id: TypeId,
    /// Whether this method is async (wraps return in `Task<>`).
    pub is_async: bool,
    /// How the trampoline should treat this method's `Result<T, E>` return, if any.
    /// `None` when the return is not a Result.
    pub result: Option<ResultKind>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Interface {
    pub name: String,
    pub emission: Emission,
    pub kind: InterfaceKind,
    pub methods: Vec<Method>,
}
