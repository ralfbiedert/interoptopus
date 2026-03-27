//! C#-specific backend patterns, e.g., magic exception handling.
use interoptopus::ffi;

mod exception;
pub use exception::{Exception, ExceptionError};

/// Magic return type that enables transparent exception mapping.
pub type Try<T> = ffi::Result<T, ExceptionError>;

// This is sort of a hack until we get custom `?` handling in Rust.
/// Helper to convert `Try<T>` to `Result<T, ExceptionError>`.
pub trait TryExtension<T> {
    fn ok(self) -> Result<T, ExceptionError>;
}

impl<T> TryExtension<T> for Try<T> {
    fn ok(self) -> Result<T, ExceptionError> {
        match self {
            Self::Ok(t) => Ok(t),
            Self::Err(e) => Err(e),
            Self::Panic => Err(ExceptionError::unknown()),
            Self::Null => Err(ExceptionError::unknown()),
        }
    }
}

/// Checks whether a string looks like `System.Exception` or `System.IO.IOException`.
const fn assert_looks_like_exception_name(fqp: &str) {
    let bytes = fqp.as_bytes();
    assert!(!bytes.is_empty() && bytes[0].is_ascii_uppercase(), "Exceptions must look like `System.Exception` or `Company.System.OtherException`");
    let mut has_dot = false;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'.' {
            has_dot = true;
            assert!(i + 1 < bytes.len() && bytes[i + 1].is_ascii_uppercase(), "Exceptions must look like `System.Exception` or `Company.System.OtherException`");
        }
        i += 1;
    }
    assert!(has_dot, "fqp must contain at least one dot");
}
