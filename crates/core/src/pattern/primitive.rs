//! Additional support for primitives like `bool`.

use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{Docs, Emission, Visibility};
use crate::lang::types::{SerializationError, Type, TypeInfo, TypeKind, TypePattern, WireIO};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::ops::Not;
use std::os::raw::c_char;

/// A single-byte boolean value where `1` means `true`, and `0` is `false`.
///
/// Other values (`2` ..= `255`) will also map to `false`(!) and [`Bool::is_strange`] will
/// signal `true`, but no undefined behavior will be triggered.
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Eq, Default, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Eq, Default))]
pub struct Bool {
    value: u8,
}

impl Bool {
    pub const TRUE: Self = Self { value: 1 };
    pub const FALSE: Self = Self { value: 0 };

    #[must_use]
    pub fn is(self) -> bool {
        self.into()
    }

    /// If a value not `0` or `1` was found.
    #[must_use]
    pub const fn is_strange(self) -> bool {
        self.value > 1
    }
}

impl Not for Bool {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is() { Self::FALSE } else { Self::TRUE }
    }
}

impl From<bool> for Bool {
    fn from(x: bool) -> Self {
        if x { Self::TRUE } else { Self::FALSE }
    }
}

impl From<Bool> for bool {
    fn from(x: Bool) -> Self {
        x.value == 1
    }
}

impl TypeInfo for Bool {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0x4ECDA35B6792FC1A61E444B3A9D3B3B8)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::Bool)
    }

    fn ty() -> Type {
        Type { name: "Bool".to_string(), visibility: Visibility::Public, docs: Docs::empty(), emission: Emission::Builtin, kind: Self::kind() }
    }

    fn register(inventory: &mut Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl WireIO for Bool {
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

/// A wrapper for the `c_char` type to differentiate it from a signed 8-bit integer for platforms
/// that support this type.
///
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Eq, Default, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Eq, Default))]
pub struct CChar {
    value: c_char,
}

impl CChar {
    pub const MAX: Self = Self { value: c_char::MAX };
    pub const MIN: Self = Self { value: c_char::MIN };
}

impl From<c_char> for CChar {
    fn from(x: c_char) -> Self {
        Self { value: x }
    }
}

impl From<CChar> for c_char {
    fn from(x: CChar) -> Self {
        x.value
    }
}

impl TypeInfo for CChar {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0xBCA68B86EF7B3FEFAFBB645D6B156754)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::CChar)
    }

    fn ty() -> Type {
        Type { name: "CChar".to_string(), visibility: Visibility::Public, docs: Docs::empty(), emission: Emission::Builtin, kind: Self::kind() }
    }

    fn register(inventory: &mut Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl WireIO for CChar {
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
    use crate::pattern::primitive::Bool;
    use crate::pattern::primitive::CChar;
    use std::os::raw::c_char;

    #[test]
    fn bool_works() {
        assert!(Bool::TRUE.is());
        assert!(!Bool::FALSE.is());
    }

    #[test]
    fn cchar_works() {
        assert!(c_char::from(CChar::MAX) == c_char::MAX);
        assert!(CChar::from(c_char::MAX) == CChar::MAX);
    }
}
