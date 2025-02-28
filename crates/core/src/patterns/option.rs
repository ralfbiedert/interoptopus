//! Like a regular [`Option`] but FFI safe.
//!
//! # Example
//!
//! This function accepts an FFI option and converts it into a Rust option.
//!
//! ```
//! use interoptopus::ffi_function;
//! use interoptopus::patterns::option::FFIOption;
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn set_value(x: FFIOption<u8>) {
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

use crate::patterns::primitives::FFIBool;
use crate::patterns::TypePattern;
use crate::util::capitalize_first_letter;
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
pub struct FFIOption<T> {
    t: T,
    is_some: FFIBool,
}

impl<T> FFIOption<T> {
    pub const fn some(data: T) -> Self {
        Self { is_some: FFIBool::TRUE, t: data }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn into_option(self) -> Option<T> {
        if self.is_some.is() {
            Some(self.t)
        } else {
            None
        }
    }

    pub fn as_ref(&self) -> Option<&T> {
        if self.is_some.is() {
            Some(&self.t)
        } else {
            None
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.is_some.is() {
            Some(&mut self.t)
        } else {
            None
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

impl<T: Default> FFIOption<T> {
    #[must_use]
    pub fn none() -> Self {
        Self { is_some: FFIBool::FALSE, t: T::default() }
    }
}

impl<T: Default> From<Option<T>> for FFIOption<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Option::None => Self::none(),
            Option::Some(t) => Self::some(t),
        }
    }
}

unsafe impl<T> CTypeInfo for FFIOption<T>
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
    use crate::patterns::option::FFIOption;

    #[test]
    fn can_create() {
        assert!(FFIOption::some(100).is_some());
        assert!(FFIOption::<u8>::none().is_none());
    }
}
