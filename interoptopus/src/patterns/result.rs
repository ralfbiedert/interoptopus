//! Like a [`Result`] but FFI safe.
use crate::lang::c::{CType, CompositeType, Documentation, Field, PrimitiveType, Visibility};
use crate::lang::rust::CTypeInfo;

use crate::patterns::primitives::FFIBool;
use crate::patterns::TypePattern;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait FFIError {
    /// The variant to return when everything went OK, usually the variant with value `0`.
    const SUCCESS: Self;
    /// Signals a null pointer was passed where an actual element was needed.
    const NULL: Self;
    /// The panic variant. Once this is observed no further calls should be attempted.
    const PANIC: Self;
}

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Default, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Default))]
pub struct FFIResult<T, E>
where
    E: FFIError,
{
    t: T,
    e: E,
}

impl<T, E> FFIResult<T> {}
