//! FFI-safe [`Option<T>`](std::option::Option) represented as a discriminated union.
//!
//! [`Option<T>`] is a `repr(C)` enum with `None` and `Some(T)` variants,
//! usable anywhere a regular Rust `Option` would be. Backends that support
//! the pattern generate idiomatic nullable / optional types.
//!
//! # Example
//!
//! ```
//! use interoptopus::ffi;
//!
//! #[ffi]
//! pub fn set_value(x: ffi::Option<u8>) {
//!     let _rust_opt: Option<u8> = x.into_option();
//! }
//! ```
//!

use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::Visibility;
use crate::wire::SerializationError;
use crate::lang::types::{TypeInfo, TypeKind, WireIO};
use std::io::{Read, Write};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An option-like type at the FFI boundary where a regular [`Option`] doesn't work.
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Eq, Default, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Eq, Default))]
#[must_use]
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

unsafe impl<T: TypeInfo> TypeInfo for Option<T> {
    const WIRE_SAFE: bool = T::WIRE_SAFE;
    const RAW_SAFE: bool = T::RAW_SAFE;
    const ASYNC_SAFE: bool = T::ASYNC_SAFE;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0xF613EA2C1CDBE74FFFAC69753255D6DE).derive_id(T::id())
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(crate::lang::types::TypePattern::Option(T::id()))
    }

    fn ty() -> crate::lang::types::Type {
        let t = T::ty();
        crate::lang::types::Type {
            emission: crate::lang::meta::common_or_module_emission(&[t.emission]),
            docs: crate::lang::meta::Docs::empty(),
            visibility: Visibility::Public,
            name: format!("Option<{}>", t.name),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl Inventory) {
        // Ensure base type is registered.
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl<T: WireIO> WireIO for Option<T> {
    fn write(&self, out: &mut impl Write) -> Result<(), SerializationError> {
        match self {
            Self::None => 0u8.write(out),
            Self::Some(v) => {
                1u8.write(out)?;
                v.write(out)
            }
        }
    }

    fn read(input: &mut impl Read) -> Result<Self, SerializationError> {
        match u8::read(input)? {
            0 => Ok(Self::None),
            _ => Ok(Self::Some(T::read(input)?)),
        }
    }

    fn live_size(&self) -> usize {
        1 + self.as_ref().map_or(0, WireIO::live_size)
    }
}
