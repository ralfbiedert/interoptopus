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
    /// If the original return type was `Result<T, ErrorXXX>` (a `Try<T>`), stores the
    /// Result's C# `TypeId` so trampolines can wrap calls with `FromCall`. When set,
    /// `rval_id` holds the unwrapped `T` instead of the full Result type.
    pub unwrapped_result_id: Option<TypeId>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Interface {
    pub name: String,
    pub emission: Emission,
    pub kind: InterfaceKind,
    pub methods: Vec<Method>,
}
