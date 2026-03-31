pub mod csharp;
pub mod kind;
mod overload;

use crate::lang::meta::{Emission, Visibility};
use crate::lang::types::kind::TypeKind;
pub use overload::{DelegateFamily, OverloadFamily, PointerFamily};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ManagedConversion {
    /// Primitive types that convert via language built-ins, and blittable types that require no
    /// marshalling logic.
    AsIs,
    /// Conversion via `To...` methods, indicating no ownership transfer (Rust type is `Copy`).
    To,
    /// Conversion via `Into...` methods, indicating ownership transfer (Rust type has `Drop`).
    Into,
}

#[derive(Clone, Debug)]
pub enum MarshalAs {
    LPStr,
}

impl std::fmt::Display for MarshalAs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LPStr => write!(f, "UnmanagedType.LPStr"),
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
            Self::Ref => write!(f, "ref"),
            Self::Out => write!(f, "out"),
            Self::MarshalAs(m) => write!(f, "[MarshalAs({m})]"),
        }
    }
}

/// Decorators for function / method return values.
#[derive(Clone, Debug)]
pub enum RvalDecorator {
    MarshalAs(MarshalAs),
    MarshalUsing(String),
}

/// Can decorate a type with additional attributes in generated C# code.
#[derive(Clone, Debug, Default)]
pub struct Decorators {
    pub param: Option<ParamDecorator>,
    pub rval: Option<RvalDecorator>,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub emission: Emission,
    pub name: String,
    pub visibility: Visibility,
    pub docs: Vec<String>,
    pub kind: TypeKind,
    pub decorators: Decorators,
}
