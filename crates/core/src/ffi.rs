//! FFI-safe versions of common std Rust types.

pub use crate::pattern::cstr::CStrPointer;
pub use crate::pattern::option::Option;
pub use crate::pattern::primitive::{Bool, CChar};
pub use crate::pattern::result::{Err, Ok, Result};
pub use crate::pattern::slice::{Slice, SliceMut};
pub use crate::pattern::string::String;

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
