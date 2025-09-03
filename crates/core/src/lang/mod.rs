//! Metadata for the Rust and C representation of language items.
//!
//! Normal users of Interoptopus probably won't have to concern
//! themselves with any of the items in this module.

use crate::pattern::TypePattern;
use std::collections::HashSet;

use crate::lang::util::{capitalize_first_letter, types_from_type_recursive};
use crate::pattern::callback::AsyncCallback;
use crate::pattern::result::ResultType;
pub use array::Array;
pub use composite::{Composite, Field, Layout, Opaque, Representation};
pub use constant::{Constant, ConstantValue};
pub use enums::{Enum, Variant, VariantKind};
pub use fnpointer::FnPointer;
pub use function::{Function, Parameter, Signature, SugaredReturnType};
pub use info::{ConstantInfo, FunctionInfo, TypeInfo};
pub use meta::{Docs, Meta, Visibility};
pub use namespace::NamespaceMappings;
pub use primitive::{Primitive, PrimitiveValue};
pub use wire::WireInfo;

mod array;
mod composite;
mod constant;
mod enums;
mod fnpointer;
mod function;
mod info;
mod meta;
mod namespace;
mod primitive;
pub mod util;
mod wire;

/// The namespace used by common types, e.g., `ffi::String`.
pub const NAMESPACE_COMMON: &str = "_common";

/// A type that can exist at the FFI boundary.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Type {
    Primitive(Primitive),
    Array(Array),
    Enum(Enum),
    Opaque(Opaque),
    Composite(Composite),
    /// A composite type, serialized and deserialized through the FFI boundary. `Wire<T>` on Rust side.
    Wire(Composite), // TODO: is this a Pattern?
    /// A type, serialized and deserialized through the FFI boundary. These are all types used from
    /// `Wire<T>` but not including this type itself. These types do not require full `Wire<T>` framework,
    /// only to be able to serialize and deserialize themselves through a buffer.
    WirePayload(WirePayload),
    FnPointer(FnPointer),
    ReadPointer(Box<Type>),
    ReadWritePointer(Box<Type>),
    /// Special patterns with primitives existing on C-level but special semantics.
    /// useful to higher level languages.
    Pattern(TypePattern),
    /// A type only known by name expected to be defined elsewhere and included.
    ExternType(String),
}

/// The type contained inside a [`Wire`](crate::wire::Wire).
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WirePayload {
    Composite(Composite),
    String,
    Enum(Enum),
    Option(Box<Type>),
    Vec(Box<Type>),
    Map(Box<Type>, Box<Type>),
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
                Primitive::Usize => std::mem::size_of::<usize>(),
                Primitive::I8 => 1,
                Primitive::I16 => 2,
                Primitive::I32 => 4,
                Primitive::I64 => 8,
                Primitive::Isize => std::mem::size_of::<isize>(),
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

    #[must_use]
    pub fn to_type(&self) -> Self {
        self.clone()
    }

    /// Produces a name unique for that type with respect to this library.
    ///
    /// The name here is supposed to uniquely determine a type relative to a library [`Inventory`](crate::inventory::Inventory).
    ///
    /// Backends may instead match on the [`Type`] variant and determine a more appropriate
    /// name on a case-by-case basis; including changing a name entirely.
    #[must_use]
    pub fn name_within_lib(&self) -> String {
        match self {
            Self::Primitive(x) => x.rust_name().to_string(),
            Self::Enum(x) => x.rust_name().to_string(),
            Self::Opaque(x) => x.rust_name().to_string(),
            Self::Composite(x) => x.rust_name().to_string(),
            Self::Wire(x) => x.rust_name().to_string(),
            Self::WirePayload(dom) => match dom {
                WirePayload::Composite(x) => x.rust_name().to_string(),
                WirePayload::String => "String".to_string(),
                WirePayload::Enum(x) => x.rust_name().to_string(),
                WirePayload::Option(x) => format!("Option{}", capitalize_first_letter(x.name_within_lib().as_str())),
                WirePayload::Vec(x) => format!("Vec{}", capitalize_first_letter(x.name_within_lib().as_str())),
                WirePayload::Map(k, v) => {
                    format!("Map{}To{}", capitalize_first_letter(k.name_within_lib().as_str()), capitalize_first_letter(v.name_within_lib().as_str()))
                }
            },
            Self::FnPointer(x) => x.rust_name(),
            Self::ReadPointer(x) => format!("ConstPtr{}", capitalize_first_letter(x.name_within_lib().as_str())),
            Self::ReadWritePointer(x) => format!("MutPtr{}", capitalize_first_letter(x.name_within_lib().as_str())),
            Self::Pattern(x) => match x {
                TypePattern::Bool => "Bool".to_string(),
                _ => x.fallback_type().name_within_lib(),
            },
            Self::Array(x) => x.rust_name(),
            Self::ExternType(name) => name.clone(),
        }
    }

    /// Lists all _other_ types this type refers to.
    #[must_use]
    pub fn embedded_types(&self) -> Vec<Self> {
        let mut hash_set: HashSet<Self> = HashSet::new();

        types_from_type_recursive(self, &mut hash_set);

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
            Self::Wire(_) => None,
            Self::WirePayload(_) => todo!(),
            Self::FnPointer(_) => None,
            Self::ReadPointer(x) => Some(x.as_ref()),
            Self::ReadWritePointer(x) => Some(x.as_ref()),
            Self::Pattern(_) => None,
            Self::Array(_) => None,
            Self::ExternType(_) => None,
        }
    }

    /// Convenience method attempting to convert the contained type as a composite.
    #[must_use]
    pub const fn as_composite_type(&self) -> Option<&Composite> {
        match self {
            Self::Composite(x) => Some(x),
            Self::Wire(x) => Some(x),
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

    #[must_use]
    pub const fn as_result_type(&self) -> Option<&ResultType> {
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

    /// Convenience method attempting to convert the contained type as a composite.
    #[must_use]
    pub const fn as_result(&self) -> Option<&ResultType> {
        match self {
            Self::Pattern(TypePattern::Result(x)) => Some(x),
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
            Self::Array(t) => t.the_type().namespace(),
            Self::Enum(t) => Some(t.meta().module()),
            Self::Opaque(t) => Some(t.meta().module()),
            Self::Composite(t) => Some(t.meta().module()),
            Self::Wire(t) => Some(t.meta().module()),
            Self::WirePayload(d) => match d {
                WirePayload::Composite(t) => Some(t.meta().module()),
                WirePayload::String => None,
                WirePayload::Enum(t) => Some(t.meta().module()),
                WirePayload::Option(t) => t.namespace(),
                WirePayload::Vec(t) => t.namespace(),
                WirePayload::Map(_, _) => todo!(),
            },
            Self::Pattern(TypePattern::NamedCallback(t)) => Some(t.meta().module()),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_wired(&self) -> bool {
        matches!(self, Self::Wire(_))
    }
}
