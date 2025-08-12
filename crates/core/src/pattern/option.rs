//! Like a regular [`Option`](std::option::Option), but FFI safe.
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

use crate::backend::capitalize_first_letter;
use crate::lang::{Docs, Enum, Meta, Representation, Type, Variant, VariantKind};
use crate::lang::{Layout, TypeInfo};
use crate::pattern::TypePattern;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An option-like type at the FFI boundary where a regular [`Option`] doesn't work.
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Eq, Default, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Eq, Default))]
pub enum Option<T> {
    Some(T),
    #[default]
    None,
}

impl<T> Option<T> {
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_option(self) -> std::option::Option<T> {
        match self {
            Self::Some(x) => Some(x),
            Self::None => None,
        }
    }

    pub fn as_ref(&self) -> std::option::Option<&T> {
        match self {
            Self::Some(x) => Some(x),
            Self::None => None,
        }
    }

    pub fn as_mut(&mut self) -> std::option::Option<&mut T> {
        match self {
            Self::Some(x) => Some(x),
            Self::None => None,
        }
    }

    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Get the value or panic.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None`.
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(t) => t,
            Self::None => panic!("Trying to unwrap None value"),
        }
    }

    /// Get the value as a mutable reference or panic.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None`.
    #[track_caller]
    pub fn unwrap_as_mut(&mut self) -> &mut T {
        match self {
            Self::Some(t) => t,
            Self::None => panic!("Trying to unwrap None value"),
        }
    }
}

impl<T> From<std::option::Option<T>> for Option<T> {
    fn from(option: std::option::Option<T>) -> Self {
        match option {
            Some(t) => Self::Some(t),
            None => Self::None,
        }
    }
}

impl<T> From<Option<T>> for std::option::Option<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Option::Some(t) => Some(t),
            Option::None => None,
        }
    }
}

unsafe impl<T> TypeInfo for Option<T>
where
    T: TypeInfo,
{
    fn type_info() -> Type {
        let doc_t = Docs::from_line("Element if Some().");

        let variants = vec![
            Variant::new("Some".to_string(), VariantKind::Typed(0, Box::new(T::type_info())), doc_t),
            Variant::new("None".to_string(), VariantKind::Unit(1), Docs::new()),
        ];

        let doc = Docs::from_line("Option that contains Some(value) or None.");
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_module_docs(T::type_info().namespace().map_or_else(String::new, std::convert::Into::into), doc);
        let t_name = capitalize_first_letter(T::type_info().name_within_lib().as_str());
        let name = format!("Option{t_name}");
        let the_enum = Enum::new(name, variants, meta, repr);
        let option_enum = OptionType::new(the_enum);
        Type::Pattern(TypePattern::Option(option_enum))
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OptionType {
    the_enum: Enum,
}

impl OptionType {
    #[must_use]
    pub fn new(the_enum: Enum) -> Self {
        Self { the_enum }
    }

    #[must_use]
    pub fn meta(&self) -> &Meta {
        self.the_enum.meta()
    }

    #[must_use]
    pub fn t(&self) -> &Type {
        self.the_enum.variants()[0].kind().as_typed().unwrap()
    }

    #[must_use]
    pub fn the_enum(&self) -> &Enum {
        &self.the_enum
    }
}

#[cfg(test)]
mod test {
    use crate::pattern::option::Option;

    #[test]
    fn can_create() {
        assert!(Option::Some(100).is_some());
        assert!(Option::<u8>::None.is_none());
    }
}
