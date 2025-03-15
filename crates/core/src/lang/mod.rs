//! Metadata for the Rust and C representation of language items.
//!
//! A a rule of thumb, types in the [`rust`](info) module generate instances
//! of types in the [`c`](c) module.
//!
//! Normal users of Interoptopus probably won't have to concern
//! themselves with any of the items in this module.

use crate::backend::{capitalize_first_letter, ctypes_from_type_recursive};
use crate::pattern::TypePattern;
use crate::pattern::result::FFIResultType;
use std::collections::HashSet;

use crate::pattern::callback::AsyncCallback;
pub use array::Array;
pub use composite::{Composite, Field, Layout, Opaque, Representation};
pub use constant::{Constant, ConstantValue};
pub use enums::{Enum, Variant};
pub use fnpointer::FnPointer;
pub use function::{Function, FunctionSignature, Parameter, SugaredReturnType};
pub use info::{ConstantInfo, FunctionInfo, TypeInfo, VariantInfo};
pub use meta::{Documentation, Meta, Visibility};
pub use primitive::{Primitive, PrimitiveValue};

mod array;
mod composite;
mod constant;
mod enums;
mod fnpointer;
mod function;
mod info;
mod meta;
mod primitive;

/// A type that can exist at the FFI boundary.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Type {
    Primitive(Primitive),
    Array(Array),
    Enum(Enum),
    Opaque(Opaque),
    Composite(Composite),
    FnPointer(FnPointer),
    ReadPointer(Box<Type>),
    ReadWritePointer(Box<Type>),
    /// Special patterns with primitives existing on C-level but special semantics.
    /// useful to higher level languages.
    Pattern(TypePattern),
}

impl Default for Type {
    fn default() -> Self {
        Self::Primitive(Primitive::Void)
    }
}

impl Type {
    #[must_use]
    pub const fn size_of(&self) -> usize {
        match self {
            Self::Primitive(p) => match p {
                Primitive::Void => 0,
                Primitive::Bool => 1,
                Primitive::U8 => 1,
                Primitive::U16 => 2,
                Primitive::U32 => 4,
                Primitive::U64 => 8,
                Primitive::I8 => 1,
                Primitive::I16 => 2,
                Primitive::I32 => 4,
                Primitive::I64 => 8,
                Primitive::F32 => 4,
                Primitive::F64 => 8,
            },
            // TODO
            _ => 999,
        }
    }

    #[must_use]
    pub fn align_of(&self) -> usize {
        unimplemented!()
    }

    #[must_use]
    pub const fn void() -> Self {
        Self::Primitive(Primitive::Void)
    }

    /// Produces a name unique for that type with respect to this library.
    ///
    /// The name here is supposed to uniquely determine a type relative to a library ([`crate::Inventory`]).
    ///
    /// Backends may instead match on the `CType` variant and determine a more appropriate
    /// name on a case-by-case basis; including changing a name entirely.
    #[must_use]
    pub fn name_within_lib(&self) -> String {
        match self {
            Self::Primitive(x) => x.rust_name().to_string(),
            Self::Enum(x) => x.rust_name().to_string(),
            Self::Opaque(x) => x.rust_name().to_string(),
            Self::Composite(x) => x.rust_name().to_string(),
            Self::FnPointer(x) => x.rust_name(),
            Self::ReadPointer(x) => format!("ConstPtr{}", capitalize_first_letter(x.name_within_lib().as_str())),
            Self::ReadWritePointer(x) => format!("MutPtr{}", capitalize_first_letter(x.name_within_lib().as_str())),
            Self::Pattern(x) => match x {
                TypePattern::Bool => "Bool".to_string(),
                _ => x.fallback_type().name_within_lib(),
            },
            Self::Array(x) => x.rust_name(),
        }
    }

    /// Lists all _other_ types this type refers to.
    #[must_use]
    pub fn embedded_types(&self) -> Vec<Self> {
        let mut hash_set: HashSet<Self> = HashSet::new();

        ctypes_from_type_recursive(self, &mut hash_set);

        hash_set.remove(self);
        hash_set.iter().cloned().collect()
    }

    /// If this were a pointer, tries to deref it and return the inner type.
    #[must_use]
    pub fn try_deref_pointer(&self) -> Option<&Self> {
        match self {
            Self::Primitive(_) => None,
            Self::Enum(_) => None,
            Self::Opaque(_) => None,
            Self::Composite(_) => None,
            Self::FnPointer(_) => None,
            Self::ReadPointer(x) => Some(x.as_ref()),
            Self::ReadWritePointer(x) => Some(x.as_ref()),
            Self::Pattern(_) => None,
            Self::Array(_) => None,
        }
    }

    /// Convenience method attempting to convert the contained type as a composite.
    #[must_use]
    pub const fn as_composite_type(&self) -> Option<&Composite> {
        match self {
            Self::Composite(x) => Some(x),
            _ => None,
        }
    }

    /// Convenience method attempting to convert the contained type as an opaque.
    #[must_use]
    pub const fn as_opaque_type(&self) -> Option<&Opaque> {
        match self {
            Self::Opaque(x) => Some(x),
            _ => None,
        }
    }

    /// Convenience method attempting to convert the contained type as a composite.
    #[must_use]
    pub const fn as_result(&self) -> Option<&FFIResultType> {
        match self {
            Self::Pattern(TypePattern::Result(x)) => Some(x),
            _ => None,
        }
    }

    /// Convenience method attempting to convert the contained type as an Async callback.
    #[must_use]
    pub const fn as_async_callback(&self) -> Option<&AsyncCallback> {
        match self {
            Self::Pattern(TypePattern::AsyncCallback(x)) => Some(x),
            _ => None,
        }
    }

    /// Convenience method attempting to get the pointer target of a contained type.
    #[must_use]
    pub const fn pointer_target(&self) -> Option<&Self> {
        match self {
            Self::ReadPointer(x) => Some(x),
            Self::ReadWritePointer(x) => Some(x),
            _ => None,
        }
    }

    /// Checks if this is a [`Primitive::Void`].
    #[must_use]
    pub const fn is_void(&self) -> bool {
        matches!(self, Self::Primitive(Primitive::Void))
    }

    /// Returns the namespace of the type.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        match self {
            Self::Array(t) => t.array_type().namespace(),
            Self::Enum(t) => Some(t.meta().namespace()),
            Self::Opaque(t) => Some(t.meta().namespace()),
            Self::Composite(t) => Some(t.meta().namespace()),
            Self::Pattern(TypePattern::NamedCallback(t)) => Some(t.meta().namespace()),
            _ => None,
        }
    }
}
