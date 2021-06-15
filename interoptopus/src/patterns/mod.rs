use crate::lang::c::{CType, PrimitiveType};

pub mod ffioption;
pub mod ascii_pointer;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TypePattern {
    AsciiPointer
}

impl TypePattern {
    /// For languages like C that don't care about these patterns, give the
    /// C-equivalent fallback type.
    pub fn fallback_type(&self) -> CType {
        match self {
            TypePattern::AsciiPointer => CType::ReadPointer(Box::new(CType::Primitive(PrimitiveType::U8)))
        }
    }
}

// pub struct PatternAsciiPointer {
//     ptr_field: String
// }

