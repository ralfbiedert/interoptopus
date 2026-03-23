use crate::lang::functions::Signature;
use crate::lang::FunctionId;
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
    /// Resolved C# return type name (e.g. `"uint"`, `"Task<uint>"`, `"void"`).
    pub rval_name: String,
}

pub struct Interface {
    pub name: String,
    pub emission: Emission,
    pub kind: InterfaceKind,
    pub methods: Vec<Method>,
}
