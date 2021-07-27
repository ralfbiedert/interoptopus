//! For return enums with defined `Ok` variants, translating to exceptions if not met.

use crate::lang::c::{EnumType, Variant};
use crate::util::log_error;
use std::error::Error;
use std::fmt::Debug;
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
    /// The variant to return when everything went OK, usually the variant with value `0`.
    const SUCCESS: Self;
    /// Signals a null pointer was passed where an actual element was needed.
    const NULL: Self;
    /// The panic variant. Once this is observed no further calls should be attempted.
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

/// Helper to transform [`Result`] types to [`Success`] enums inside `extern "C"` functions.
///
/// This function executes the given closure `f`. If `f` returns `Ok(())` the `SUCCESS`
/// variant is returned. On a panic or `Err` the respective error variant is returned instead.
///
/// # Feature Flags
///
/// If the `log` crate option is enabled this will invoke `log::error` on errors.
///
/// # Safety
///
/// Once [`Success::PANIC`] has been observed the enum's recipient should stop calling this API
/// (and probably gracefully shutdown or restart), as any subsequent call risks causing a
/// process abort.
#[allow(unused_variables)]
pub fn panics_and_errors_to_ffi_enum<E: Error, FE: Success>(f: impl FnOnce() -> Result<(), E>, error_context: &str) -> FE
where
    FE: From<Result<(), E>>,
{
    let result: Result<(), E> = match std::panic::catch_unwind(AssertUnwindSafe(|| f())) {
        Ok(x) => x,
        Err(_) => {
            log_error(|| format!("Panic observed near {}", error_context));
            return FE::PANIC;
        }
    };

    if let Err(e) = &result {
        log_error(|| format!("Error observed near {}: {}", error_context, e.to_string()));
    }

    result.into()
}

// /// # Example
// ///
// /// ```
// /// use interoptopus::patterns::success_enum::panics_and_errors_to_ffi_enum;
// /// use interoptopus::here;
// ///
// /// #[no_mangle]
// /// extern "C" fn f() {
// ///     panics_and_errors_to_ffi_enum(|| {
// ///         panic!("Will be caught.")
// ///     }, here!())
// /// }
// /// ```
//
