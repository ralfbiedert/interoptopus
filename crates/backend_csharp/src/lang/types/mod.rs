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

/// Decorators for function / method parameters.
#[derive(Clone, Debug)]
pub enum ParamDecorator {
    Ref,
    Out,
    MarshalAs(MarshalAs),
}

impl ParamDecorator {
    pub fn to_string(&self) -> String {
        match self {
            ParamDecorator::Ref => "ref".to_string(),
            ParamDecorator::Out => "out".to_string(),
            ParamDecorator::MarshalAs(m) => format!("[MarshalAs({})]", m),
        }
    }
}

#[derive(Clone, Debug)]
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
