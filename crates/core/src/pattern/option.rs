//! Like a regular [`Option`](std::option::Option), but FFI safe.
//!
//! # Example
//!
//! This function accepts an FFI option and converts it into a Rust option.
//!
//! ```
//! use interoptopus::ffi;
//!
//! #[ffi]
//! pub fn set_value(x: ffi::Option<u8>) {
//!     let _ = x.into_option();
//! }
//! ```
//!

use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{Emission, Visibility};
use crate::lang::types::{SerializationError, TypeInfo};
use crate::lang::types::{TypeKind, WireIO};
use std::io::{Read, Write};

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

impl<T: TypeInfo> TypeInfo for Option<T> {
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
            emission: Emission::Common,
            docs: crate::lang::meta::Docs::empty(),
            visibility: Visibility::Public,
            name: format!("Option<{}>", t.name),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut Inventory) {
        // Ensure base type is registered.
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl<T: WireIO> WireIO for Option<T> {
    fn write(&self, _: &mut impl Write) -> Result<(), SerializationError> {
        todo!()
    }

    fn read(_: &mut impl Read) -> Result<Self, SerializationError> {
        todo!()
    }

    fn live_size(&self) -> usize {
        todo!()
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
