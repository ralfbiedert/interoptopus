//! Like a regular [`Option`] but FFI safe.
//!
//! # Example
//!
//! This function accepts an FFI option and converts it into a Rust option.
//!
//! ```
//! use interoptopus::ffi_function;
//! use interoptopus::ffi;
//!
//! #[ffi_function]
//! pub fn set_value(x: ffi::Option<u8>) {
//!     let _ = x.into_option();
//! }
//! ```
//!
//! On C FFI level the following binding code is generated:
//!
//! ```c
//! typedef struct optionu8
//!     {
//!     uint8_t t;
//!     uint8_t is_some;
//!     } optionu8;
//!
//! void set_value(optionu8 x);
//! ```
//!
use crate::lang::c::{CType, CompositeType, Documentation, Field, Layout, Meta, PrimitiveType, Representation, Visibility};
use crate::lang::rust::CTypeInfo;

use crate::backend::util::capitalize_first_letter;
use crate::pattern::TypePattern;
use crate::pattern::primitive::Bool;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An option-like type at the FFI boundary where a regular [`Option`] doesn't work.
///
/// # C API
///
/// The option will be considered `Some` if and only if `is_some` is `1`. All
/// other values mean `None`.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Eq, Default, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Eq, Default))]
pub struct Option<T> {
    t: T,
    is_some: Bool,
}

impl<T> Option<T> {
    pub const fn some(data: T) -> Self {
        Self { is_some: Bool::TRUE, t: data }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn into_option(self) -> std::option::Option<T> {
        if self.is_some.is() {
            std::option::Option::Some(self.t)
        } else {
            std::option::Option::None
        }
    }

    pub fn as_ref(&self) -> std::option::Option<&T> {
        if self.is_some.is() {
            std::option::Option::Some(&self.t)
        } else {
            std::option::Option::None
        }
    }

    pub fn as_mut(&mut self) -> std::option::Option<&mut T> {
        if self.is_some.is() {
            std::option::Option::Some(&mut self.t)
        } else {
            std::option::Option::None
        }
    }

    pub fn is_some(&self) -> bool {
        self.is_some.is()
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Get the value or panic.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None`.
    #[track_caller]
    pub fn unwrap(self) -> T {
        if self.is_some.is() {
            self.t
        } else {
            panic!("Trying to unwrap None value");
        }
    }

    /// Get the value as a mutable reference or panic.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None`.
    #[track_caller]
    pub fn unwrap_as_mut(&mut self) -> &mut T {
        if self.is_some.is() {
            &mut self.t
        } else {
            panic!("Trying to unwrap None value");
        }
    }
}

impl<T: Default> Option<T> {
    #[must_use]
    pub fn none() -> Self {
        Self { is_some: Bool::FALSE, t: T::default() }
    }
}

impl<T: Default> From<std::option::Option<T>> for Option<T> {
    fn from(option: std::option::Option<T>) -> Self {
        match option {
            std::option::Option::None => Self::none(),
            std::option::Option::Some(t) => Self::some(t),
        }
    }
}

unsafe impl<T> CTypeInfo for Option<T>
where
    T: CTypeInfo,
{
    fn type_info() -> CType {
        let doc_t = Documentation::from_line("Element that is maybe valid.");
        let doc_is_some = Documentation::from_line("Byte where `1` means element `t` is valid.");

        let fields = vec![
            Field::with_documentation("t".to_string(), T::type_info(), Visibility::Private, doc_t),
            Field::with_documentation("is_some".to_string(), CType::Primitive(PrimitiveType::U8), Visibility::Private, doc_is_some),
        ];

        let doc = Documentation::from_line("Option type containing boolean flag and maybe valid data.");
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_namespace_documentation(T::type_info().namespace().map_or_else(String::new, std::convert::Into::into), doc);
        let name = capitalize_first_letter(T::type_info().name_within_lib().as_str());
        let composite = CompositeType::with_meta_repr(format!("Option{name}"), fields, meta, repr);
        CType::Pattern(TypePattern::Option(composite))
    }
}

#[cfg(test)]
mod test {
    use crate::pattern::option::Option;

    #[test]
    fn can_create() {
        assert!(Option::some(100).is_some());
        assert!(Option::<u8>::none().is_none());
    }
}
