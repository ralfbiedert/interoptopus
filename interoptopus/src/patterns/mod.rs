//! Optional types that translate to binding with better semantics in languages supporting them.
//!
//! Patterns are purely optional. If you want to use a certain pattern in your bindings
//! you generally define one or more functions that use some of the types contained in this module.
//!
//! Backends which support a pattern will then generate _additional_ language-specific helpers
//! and bindings  for it. In any case, regardless whether a pattern is supported by a backend or not,
//! fallback bindings will be available.
//!
//! ## Pattern Support
//!
//! Patterns are exclusively designed _on top of_ existing, C-compatible functions and types.
//! That means a backend will handle a pattern in one of three ways:
//!
//! - The pattern is **supported** and the backend will generate the raw, underlying type and / or
//! a language-specific abstraction that safely and conveniently handles it. Examples
//! include converting an [`AsciiPointer`](ascii_pointer) to a C# `string`, or a [`class`](crate::patterns::class)
//! to a Python `class`.
//!
//! - The pattern is not supported and will be omitted **if the pattern was merely an aggregate** of
//! existing items. Examples include the [`class`](crate::patterns::class) pattern in C which will not
//! be emitted. However, this will not pose a problem as all constituent types and methods (functions)
//! are still available as raw bindings.
//!
//! - The pattern is not supported and will be **replaced with a fallback type**. Examples include
//! the [`AsciiPointer`](ascii_pointer) which will become a regular `*const u8` in C.
//!
//! In other words, regardless of which pattern was used, the involved methods and types will always
//! be accessible from any language.

use crate::lang::c::{CType, CompositeType, PrimitiveType};
use crate::patterns::class::Class;
use crate::patterns::success_enum::SuccessEnum;

pub mod ascii_pointer;
pub mod callbacks;
pub mod class;
pub mod option;
pub mod slice;
pub mod success_enum;

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
    Slice(CompositeType),
}

impl TypePattern {
    /// For languages like C that don't care about these patterns, give the
    /// C-equivalent fallback type.
    ///
    /// This function will never return a [`CType::Pattern`] variant.
    pub fn fallback_type(&self) -> CType {
        match self {
            TypePattern::AsciiPointer => CType::ReadPointer(Box::new(CType::Primitive(PrimitiveType::U8))),
            TypePattern::SuccessEnum(e) => CType::Enum(e.the_enum().clone()),
            TypePattern::Slice(x) => CType::Composite(x.clone()),
        }
    }
}
