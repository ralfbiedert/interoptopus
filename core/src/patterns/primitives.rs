//! Additional support for primitives like `bool`.

use crate::lang::c::CType;
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Not;

/// A single-byte boolean value where `1` means `true`, and `0` is `false`.
///
/// Other values (`2` ..= `255`) will also map to `false`(!) and [`FFIBool::is_strange`] will
/// signal `true`, but no undefined behavior will be triggered.
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Default, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Default))]
pub struct FFIBool {
    value: u8,
}

impl FFIBool {
    pub const TRUE: FFIBool = FFIBool { value: 1 };
    pub const FALSE: FFIBool = FFIBool { value: 0 };

    pub fn is(self) -> bool {
        self.into()
    }

    /// If a value not `0` or `1` was found.
    pub fn is_strange(self) -> bool {
        self.value > 1
    }
}

unsafe impl CTypeInfo for FFIBool {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::Bool)
    }
}

impl Not for FFIBool {
    type Output = FFIBool;

    fn not(self) -> Self::Output {
        match self.is() {
            true => FFIBool::FALSE,
            false => FFIBool::TRUE,
        }
    }
}

impl From<bool> for FFIBool {
    fn from(x: bool) -> Self {
        if x {
            Self::TRUE
        } else {
            Self::FALSE
        }
    }
}

impl From<FFIBool> for bool {
    fn from(x: FFIBool) -> Self {
        x.value == 1
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::primitives::FFIBool;

    #[test]
    fn bool_works() {
        assert!(FFIBool::TRUE.is());
        assert!(!FFIBool::FALSE.is());
    }
}
