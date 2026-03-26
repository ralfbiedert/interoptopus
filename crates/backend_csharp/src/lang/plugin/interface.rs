use crate::lang::FunctionId;
use crate::lang::TypeId;
use crate::lang::functions::Signature;
use interoptopus::lang::meta::Emission;

pub enum InterfaceKind {
    Plugin,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
    Regular,
    Static,
}

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
}

pub struct Interface {
    pub name: String,
    pub emission: Emission,
    pub kind: InterfaceKind,
    pub methods: Vec<Method>,
}
