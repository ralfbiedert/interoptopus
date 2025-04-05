//! Convenience patterns like [services](crate::pattern::service), [strings](crate::pattern::string), [results](crate::pattern::result::Result) and [options](crate::pattern::option::Option).
//!
//! Patterns are optional types and constructs you can use. Most patterns are automatically applied
//! once you use their corresponding type.  
//!
//! Backends which support a pattern will then generate _additional_ language-specific helpers
//! and bindings  for it. In any case, regardless whether a pattern is supported by a backend or not,
//! fallback bindings will be available.
//!
//! ## Pattern Usage
//!
//! For example, instead of accepting a `*const u8` (or similar) and returning `0` on success
//!
//! ```
//! # use interoptopus::ffi_function;
//!
//! #[ffi_function]
//! pub fn write_file(file: *const u8) -> i8 {
//!    let file = unsafe { /* ... */ };
//!    0
//! }
//!
//! ```
//!
//! you would instead accept an [`ffi::String`](crate::pattern::string::String) and return an [`ffi::Result`](crate::pattern::result::Result):
//!
//! ```
//! # use interoptopus::{ffi_function, ffi_type, ffi};
//! #
//! # #[ffi_type]
//! # pub enum MyError {
//! #    Bad
//! # }
//!
//! #[ffi_function]
//! pub fn write_file(file: ffi::String) -> ffi::Result<(), MyError> {
//!    let file = file.as_str();
//!    ffi::Ok(())
//! }
//!
//! ```
//! That way you won't have to write `unsafe` code, _and_ you get more idiomatic code in most
//! backends. For example, in C# you might end up with a simple `WriteFile("foo.txt")` call
//! that automatically converts the used `string` to UTF-8, and in turn converts a failed result
//! to a CLR exception.
//!
//! ## Pattern Backend Support
//!
//! Patterns are exclusively **designed _on top of_ existing, C-compatible functions and types**.
//! That means a backend will handle a pattern in one of three ways:
//!
//! - The pattern is **supported** and the backend will generate the raw, underlying type and / or
//!   a language-specific abstraction that safely and conveniently handles it. Examples
//!   include converting a [`String`](crate::pattern::string::String) to a C# `string`, or a [`service`]
//!   to a Python `class`.
//!
//! - The pattern is not supported and will be **omitted, if the pattern was merely an aggregate** of
//!   existing items. Examples include the [`service`] pattern in C which will not
//!   be emitted. However, this will not pose a problem as all constituent types and methods (functions)
//!   are still available as raw bindings.
//!
//! - The pattern is not supported and will be **replaced with a fallback type**. Examples include
//!   the [`CStrPointer`](string) which will become a regular `*const char` in C.
//!
//!
//! # Pattern Composition
//!
//! Due to a lack of expressiveness in other languages, patterns usually compose without issues in Rust, but
//! not in all backends. For example, something like `Slice<Result<Option<String>, Error>>` is supported in
//! Rust without issues, but its UX might suffer in Python.
use crate::lang::{Composite, Primitive, Type, TypeInfo};
use crate::pattern::builtins::Builtins;
use crate::pattern::callback::{AsyncCallback, NamedCallback};
use crate::pattern::option::OptionType;
use crate::pattern::result::ResultType;
use crate::pattern::service::ServiceDefinition;
use crate::pattern::slice::SliceType;
use crate::pattern::vec::VecType;
use std::ffi::c_char;

#[doc(hidden)]
pub mod api_entry;
pub mod api_guard;
pub mod asynk;
pub mod builtins;
pub mod callback;
pub mod cstr;
pub mod option;
pub mod primitive;
pub mod result;
pub mod service;
pub mod slice;
pub mod string;
pub mod surrogate;
pub mod vec;

/// A pattern on a library level, usually involving both methods and types.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum LibraryPattern {
    Service(ServiceDefinition),
    Builtins(Builtins),
}

/// Used mostly internally and provides pattern info for auto generated structs.
#[doc(hidden)]
pub trait LibraryPatternInfo {
    fn pattern_info() -> LibraryPattern;
}

impl From<ServiceDefinition> for LibraryPattern {
    fn from(x: ServiceDefinition) -> Self {
        Self::Service(x)
    }
}

/// A pattern on a type level.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum TypePattern {
    CStrPointer,
    Utf8String(Composite),
    APIVersion,
    Slice(SliceType),
    SliceMut(SliceType),
    Option(OptionType),
    Result(ResultType),
    Bool,
    CChar,
    NamedCallback(NamedCallback),
    AsyncCallback(AsyncCallback),
    Vec(VecType),
}

impl TypePattern {
    /// For languages like C that don't care about these patterns, give the
    /// C-equivalent fallback type.
    ///
    /// This function will never return a [`Type::Pattern`] variant.
    #[must_use]
    pub fn fallback_type(&self) -> Type {
        match self {
            Self::CStrPointer => Type::ReadPointer(Box::new(Type::Pattern(Self::CChar))),
            Self::Slice(x) => Type::Composite(x.composite_type().clone()),
            Self::SliceMut(x) => Type::Composite(x.composite_type().clone()),
            Self::Option(x) => x.the_enum().to_type(),
            Self::Result(x) => x.the_enum().to_type(),
            Self::NamedCallback(x) => Type::FnPointer(x.fnpointer().clone()),
            Self::Bool => Type::Primitive(Primitive::U8),
            Self::CChar => c_char::type_info(),
            Self::APIVersion => Type::Primitive(Primitive::U64),
            Self::AsyncCallback(x) => Type::FnPointer(x.fnpointer().clone()),
            Self::Utf8String(x) => Type::Composite(x.clone()),
            Self::Vec(x) => Type::Composite(x.composite_type().clone()),
        }
    }
}
