//! For return enums with defined `Ok` variants, translating to exceptions if not met.

use crate::lang::c::{EnumType, Variant};

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
///     OtherError = 2,
/// }
///
/// impl Success for FFIError {
///     const SUCCESS: Self = Self::Ok;
///     const NULL: Self = Self::NullPassed;
/// }
/// ```
pub trait Success {
    /// This variant
    const SUCCESS: Self;
    const NULL: Self;
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
pub fn error_to_ffi_error<E, FE: Success>(f: impl FnOnce() -> Result<(), E>) -> FE
where
    FE: From<E>,
{
    match f() {
        Ok(_) => FE::SUCCESS,
        Err(e) => e.into(),
    }
}
