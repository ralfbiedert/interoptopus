//! For return enums with defined `Ok` variants, translating to exceptions if not met.

use crate::lang::c::{EnumType, Variant};
use std::panic::AssertUnwindSafe;

/// A trait you should implement for enums that signal errors in FFI calls.
///
/// The `SUCCESS` variant will be used to automatically convert `Result::Ok`
/// values, and `NULL` when a required pointer was detected to be `null`.
///
/// # Example
///
/// ```
/// use interoptopus::patterns::success_enum::Success;
///
/// enum FFIError {
///     Ok = 0,
///     NullPassed = 1,
///     Panic = 2,
///     OtherError = 3,
/// }
///
/// impl Success for FFIError {
///     const SUCCESS: Self = Self::Ok;
///     const NULL: Self = Self::NullPassed;
///     const PANIC: Self = Self::Panic;
/// }
/// ```
pub trait Success {
    /// This variant
    const SUCCESS: Self;
    const NULL: Self;
    const PANIC: Self;
}

/// Internal helper derived for enums that are [`Success`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SuccessEnum {
    the_enum: EnumType,
    success_variant: Variant,
}

impl SuccessEnum {
    pub fn new(the_enum: EnumType, success_variant: Variant) -> Self {
        Self { the_enum, success_variant }
    }

    pub fn the_enum(&self) -> &EnumType {
        &self.the_enum
    }

    pub fn success_variant(&self) -> &Variant {
        &self.success_variant
    }
}

/// Helper to transform [`Error`] types to [`Success`] enums inside `extern "C"` functions.
pub fn panics_and_errors_to_ffi_error<E, FE: Success>(f: impl FnOnce() -> Result<(), E>) -> FE
where
    FE: From<Result<(), E>>,
{
    let result: Result<(), E> = match std::panic::catch_unwind(AssertUnwindSafe(|| f())) {
        Ok(x) => x,
        Err(_) => return FE::PANIC,
    };

    result.into()
}
