pub mod csharp;
pub mod kind;
mod overload;

use crate::lang::meta::Emission;
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

#[derive(Clone, Debug)]
pub enum MarshalAs {
    LPStr,
}

impl std::fmt::Display for MarshalAs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarshalAs::LPStr => write!(f, "UnmanagedType.LPStr"),
        }
    }
}

/// Decorators for function / method parameters.
#[derive(Clone, Debug)]
pub enum ParamDecorator {
    Ref,
    Out,
    MarshalAs(MarshalAs),
}

impl std::fmt::Display for ParamDecorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamDecorator::Ref => write!(f, "ref"),
            ParamDecorator::Out => write!(f, "out"),
            ParamDecorator::MarshalAs(m) => write!(f, "[MarshalAs({m})]"),
        }
    }
}

/// Can decorate a type with additional attributes in generated C# code.
#[derive(Clone, Debug, Default)]
pub struct Decorators {
    pub param: Option<ParamDecorator>,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub emission: Emission,
    pub name: String,
    pub kind: TypeKind,
    pub decorators: Decorators,
}
