//! Convenience patterns in supported languages.
//!
//! Patterns are purely optional. If you want to use a certain pattern in your bindings
//! you generally define one or more functions that use some of the types contained in this module.
//!
//! Backends which support a pattern will then generate _additional_ language-specific helpers
//! and bindings  for it. In any case, regardless whether a pattern is supported by a backend or not,
//! fallback bindings will be available.
//!
//! ## Pattern Usage
//!
//! Unless otherwise stated patterns are used by, well, using them in signatures. For example,
//! instead of making `x` a type `*const u8` (or similar) in the following `print` function:
//!
//! ```
//! # use interoptopus::ffi_function;
//!
//! #[ffi_function]
//! pub fn print_ptr(x: *const u8) {
//!    // Write unsafe code to convert `x`
//! }
//!
//! ```
//!
//! you would instead accept an [`CStrPointer`](crate::pattern::string::CStrPointer):
//!
//! ```
//! # use interoptopus::{ffi_function, ffi};
//! # use std::ffi::CStr;
//!
//! #[ffi_function]
//! pub fn print_ascii(x: ffi::CStrPointer) {
//!    // Call `x.as_str()` and handle Result
//! }
//!
//! ```
//!
//! This has the added benefit that any backend supporting a specific pattern will also
//! generate more **idiomatic code in the binding**. In the example above, C# might
//! emit a `ref ubyte` or `IntPtr` type for the `print_ptr`, but will use a correctly marshalled
//! `string` type for `print_ascii`.
//!
//!
//! ## Pattern Backend Support
//!
//! Patterns are exclusively **designed _on top of_ existing, C-compatible functions and types**.
//! That means a backend will handle a pattern in one of three ways:
//!
//! - The pattern is **supported** and the backend will generate the raw, underlying type and / or
//!   a language-specific abstraction that safely and conveniently handles it. Examples
//!   include converting an [`CStrPointer`](string) to a C# `string`, or a [`service`](crate::pattern::service)
//!   to a Python `class`.
//!
//! - The pattern is not supported and will be **omitted, if the pattern was merely an aggregate** of
//!   existing items. Examples include the [`service`](crate::pattern::service) pattern in C which will not
//!   be emitted. However, this will not pose a problem as all constituent types and methods (functions)
//!   are still available as raw bindings.
//!
//! - The pattern is not supported and will be **replaced with a fallback type**. Examples include
//!   the [`CStrPointer`](string) which will become a regular `*const char` in C.
//!
//! In other words, regardless of which pattern was used, the involved methods and types will always
//! be accessible from any language.
//!
//! # Pattern Composition
//!
//! Due to a lack of expressiveness in other languages, pattern composition is often limited. Things that work
//! easily in Rust (e.g., a nested `FFISlice<FFIOption<CStrPointer>>`), aren't supported in other languages.
//! You therefore should rather err on the side of conservatism when designing APIs.
//!
//! While we aim to guarantee that 'flat' patterns either work, or gracefully fall-back
//! to a more primitive representation, nested patterns through generics might simply fail to compile
//! in certain backends.
//!

use crate::lang::{CType, CompositeType, PrimitiveType, TypeInfo};
use crate::pattern::builtins::Builtins;
use crate::pattern::callback::{AsyncCallback, NamedCallback};
use crate::pattern::result::{FFIErrorEnum, FFIResultType};
use crate::pattern::service::ServiceDefinition;
use crate::pattern::slice::SliceType;
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

/// A pattern on a library level, usually involving both methods and types.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[non_exhaustive]
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
#[non_exhaustive]
pub enum TypePattern {
    CStrPointer,
    Utf8String(CompositeType),
    APIVersion,
    FFIErrorEnum(FFIErrorEnum),
    Slice(SliceType),
    SliceMut(SliceType),
    Option(CompositeType),
    Result(FFIResultType),
    Bool,
    CChar,
    NamedCallback(NamedCallback),
    AsyncCallback(AsyncCallback),
}

impl TypePattern {
    /// For languages like C that don't care about these patterns, give the
    /// C-equivalent fallback type.
    ///
    /// This function will never return a [`CType::Pattern`] variant.
    #[must_use]
    pub fn fallback_type(&self) -> CType {
        match self {
            Self::CStrPointer => CType::ReadPointer(Box::new(CType::Pattern(Self::CChar))),
            Self::FFIErrorEnum(e) => CType::Enum(e.the_enum().clone()),
            Self::Slice(x) => CType::Composite(x.composite_type().clone()),
            Self::SliceMut(x) => CType::Composite(x.composite_type().clone()),
            Self::Option(x) => CType::Composite(x.clone()),
            Self::Result(x) => CType::Composite(x.composite().clone()),
            Self::NamedCallback(x) => CType::FnPointer(x.fnpointer().clone()),
            Self::Bool => CType::Primitive(PrimitiveType::U8),
            Self::CChar => c_char::type_info(),
            Self::APIVersion => CType::Primitive(PrimitiveType::U64),
            Self::AsyncCallback(x) => CType::FnPointer(x.fnpointer().clone()),
            Self::Utf8String(x) => CType::Composite(x.clone()),
        }
    }
}
