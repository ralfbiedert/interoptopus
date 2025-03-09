//! Additional support for primitives like `bool`.

use crate::lang::c::CType;
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
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

unsafe impl CTypeInfo for Bool {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::Bool)
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

unsafe impl CTypeInfo for CChar {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::CChar)
    }
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

#[cfg(test)]
mod test {
    use crate::patterns::primitives::Bool;
    use crate::patterns::primitives::CChar;
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
