//! FFI-safe versions of common std Rust types.

pub use crate::patterns::cstr::CStrPointer;
pub use crate::patterns::option::Option;
pub use crate::patterns::primitive::{Bool, CChar};
pub use crate::patterns::result::{Err, Ok, Result};
pub use crate::patterns::slice::{Slice, SliceMut};
pub use crate::patterns::string::String;

/// Logs an error if compiled with feature `log`.
#[cfg(feature = "log")]
#[inline]
pub fn log_error<S: AsRef<str>, F: Fn() -> S>(f: F) {
    log::error!("{}", f().as_ref());
}

/// Logs an error if compiled with feature `log`.
#[cfg(not(feature = "log"))]
#[inline(always)]
pub fn log_error<S: AsRef<str>, F: Fn() -> S>(_f: F) {}
