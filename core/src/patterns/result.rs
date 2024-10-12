//! For return enums with defined `Ok` variants; may translate to exceptions if not met.
//!
//!
//! # Examples
//!
//! Functions returning a [`FFIError`] might receive special treatment in backends supporting
//! exception handling. For example, a [`service`](crate::patterns::service) method defined
//! as:
//!
//! ```
//! # use interoptopus::Error;
//! #
//! pub fn my_method() -> Result<(), Error> {
//!     Ok(())
//! }
//! ```
//!
//! might receive a binding helper equivalent to:
//!
//! ```csharp
//! public void MyMethod()
//! {
//!     var rval = Interop.simple_service_my_method(_context);
//!     if (rval != FFIError.Ok)
//!     {
//!         throw new Exception($"Something went wrong {rval}");
//!     }
//! }
//! ```

use crate::lang::c::{EnumType, Variant};
use crate::util::log_error;
use std::any::Any;
use std::fmt::Debug;
use std::panic::AssertUnwindSafe;

/// A trait you should implement for enums that signal errors in FFI calls.
///
/// Once implemented, the enum can be used in [services](crate::patterns::service) to automatically
/// convert `Result<(), E>` types to FFI enums.
///
/// # Example
///
/// ```
/// use interoptopus::patterns::result::FFIError;
/// use interoptopus::ffi_type;
///
/// // Some Error used in your application.
/// pub enum Error {
///     Bad,
/// }
///
/// // The error FFI users should see
/// #[ffi_type(error)]
/// enum MyFFIError {
///     Ok = 0,
///     NullPassed = 1,
///     Panic = 2,
///     OtherError = 3,
/// }
///
/// // Gives special meaning to some of your error variants.
/// impl FFIError for MyFFIError {
///     const SUCCESS: Self = Self::Ok;
///     const NULL: Self = Self::NullPassed;
///     const PANIC: Self = Self::Panic;
/// }
///
/// // How to map an `Error` to an `MyFFIError`.
/// impl From<Error> for MyFFIError {
///     fn from(x: Error) -> Self {
///         match x {
///             Error::Bad => Self::OtherError,
///         }
///     }
/// }
///
/// ```
pub trait FFIError: Sized {
    /// The variant to return when everything went OK, usually the variant with value `0`.
    const SUCCESS: Self;
    /// Signals a null pointer was passed where an actual element was needed.
    const NULL: Self;
    /// The panic variant. Once this is observed no further calls should be attempted.
    const PANIC: Self;

    // fn ok(self) -> Result<(), E>;
}

// #[repr(C)]
// #[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Default, Deserialize, Serialize))]
// #[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Default))]
// pub struct FFIResult<T, E>
// where
//     E: FFIError,
// {
//     t: T,
//     e: E,
// }

/// Internal helper derived for enums that are an [`FFIError`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FFIErrorEnum {
    the_enum: EnumType,
    success_variant: Variant,
    panic_variant: Variant,
}

impl FFIErrorEnum {
    pub fn new(the_enum: EnumType, success_variant: Variant, panic_variant: Variant) -> Self {
        Self {
            the_enum,
            success_variant,
            panic_variant,
        }
    }

    pub fn the_enum(&self) -> &EnumType {
        &self.the_enum
    }

    pub fn success_variant(&self) -> &Variant {
        &self.success_variant
    }

    pub fn panic_variant(&self) -> &Variant {
        &self.panic_variant
    }
}

/// Helper to transform [`Result`] types to [`FFIError::SUCCESS`] enums inside `extern "C"` functions.
///
/// This function executes the given closure `f`. If `f` returns `Ok(())` the `SUCCESS`
/// variant is returned. On a panic or `Err` the respective error variant is returned instead.
///
/// # Feature Flags
///
/// If the `log` crate option is enabled this will invoke `log::error` on errors and panics.
///
/// # Example
///
/// ```
/// use interoptopus::patterns::result::panics_and_errors_to_ffi_enum;
/// use interoptopus::{ffi_type, ffi_function, here};
/// # use std::fmt::{Display, Formatter};
/// #
/// # #[derive(Debug)]
/// # pub enum Error {
/// #     Bad,
/// # }
/// #
/// # impl Display for Error {
/// #    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
/// #        Ok(())
/// #    }
/// # }
/// #
/// # impl std::error::Error for Error {}
///
/// // The FFI error the library users will see.
/// #[ffi_type(error)]
/// pub enum MyFFIError {
///     Ok = 0,
///     Null = 100,
///     Panic = 200,
///     Fail = 300,
/// }
///
/// // How to convert a normal error to an FFI Error.
/// impl From<Error> for MyFFIError {
///     fn from(x: Error) -> Self {
///         match x {
///             Error::Bad => Self::Fail,
///         }
///     }
/// }
///
/// // Map special error conditions to your error type.
/// impl interoptopus::patterns::result::FFIError for MyFFIError {
///     const SUCCESS: Self = Self::Ok;
///     const NULL: Self = Self::Null;
///     const PANIC: Self = Self::Panic;
/// }
///
/// // Now call a function that may panic or throw an error.
/// #[ffi_function]
/// #[allow(unreachable_code)]
/// pub fn panics() -> MyFFIError {
///     panics_and_errors_to_ffi_enum(
///         || {
///             panic!("Oh no");
///             Ok::<(), Error>(())
///         },
///         here!(),
///     )
/// }
/// ```
///
/// # Safety
///
/// Once [`FFIError::PANIC`] has been observed the enum's recipient should stop calling this API
/// (and probably gracefully shutdown or restart), as any subsequent call risks causing a
/// process abort.
#[allow(unused_variables)]
pub fn panics_and_errors_to_ffi_enum<E: Debug, FE: FFIError>(f: impl FnOnce() -> Result<(), E>, error_context: &str) -> FE
where
    FE: From<E>,
{
    let result: Result<(), E> = match std::panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(x) => x,
        Err(e) => {
            log_error(|| format!("Panic in ({}): {}", error_context, get_panic_message(e.as_ref())));
            return FE::PANIC;
        }
    };

    if let Err(e) = &result {
        log_error(|| format!("Error in ({}): {:?}", error_context, e));
    }

    match result {
        Ok(_) => FE::SUCCESS,
        Err(e) => FE::from(e),
    }
}

/// Extracts a string message from a panic unwind.
pub fn get_panic_message(pan: &(dyn Any + Send)) -> &str {
    match pan.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match pan.downcast_ref::<String>() {
            Some(s) => s,
            None => "Any { .. }",
        },
    }
}
