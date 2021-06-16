use crate::lang::c::{CType, PrimitiveType};
use crate::patterns::class::Class;
use crate::patterns::successenum::SuccessEnum;

pub mod ascii_pointer;
pub mod class;
pub mod ffioption;
pub mod successenum;

/// A pattern on a library level, usually involving both methods and types.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LibraryPattern {
    Class(Class),
}

impl From<Class> for LibraryPattern {
    fn from(x: Class) -> Self {
        Self::Class(x)
    }
}

/// A pattern on a type level.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TypePattern {
    AsciiPointer,
    SuccessEnum(SuccessEnum),
}

impl TypePattern {
    /// For languages like C that don't care about these patterns, give the
    /// C-equivalent fallback type.
    pub fn fallback_type(&self) -> CType {
        match self {
            TypePattern::AsciiPointer => CType::ReadPointer(Box::new(CType::Primitive(PrimitiveType::U8))),
            TypePattern::SuccessEnum(e) => CType::Enum(e.the_enum().clone()),
        }
    }
}
