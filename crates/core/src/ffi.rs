//! FFI-safe replacements for common Rust standard library types.
//!
//! Types in this module are designed to be used with the `ffi::` prefix in function
//! signatures, keeping it clear at the call site that a type crosses the FFI boundary:
//!
//! ```rust
//! use interoptopus::ffi;
//!
//! # #[ffi]
//! # pub enum MyError { General }
//! #
//! #[ffi]
//! pub fn lookup_name(id: u32) -> ffi::Result<ffi::String, MyError> {
//!     ffi::Ok("hello".to_string().into())
//! }
//! ```
//!
//! Each type mirrors its `std` counterpart but uses a `#[repr(C)]` layout safe for
//! passing across the FFI boundary:
//!
//! | `ffi::` type       | Replaces                    |
//! |--------------------|-----------------------------|
//! | [`Option<T>`]      | `std::option::Option<T>`    |
//! | [`Result<T, E>`]   | `std::result::Result<T, E>` |
//! | [`String`]         | `std::string::String`       |
//! | [`Vec<T>`]         | `std::vec::Vec<T>`          |
//! | [`Slice<T>`]       | `&[T]`                      |
//! | [`SliceMut<T>`]    | `&mut [T]`                  |
//! | [`CStrPtr`]        | `*const c_char`             |
//! | [`Bool`]           | `bool`                      |
//! | [`CChar`]          | `c_char`                    |
//!
//! See the [reference project](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src)
//! for comprehensive usage examples.

pub use crate::pattern::cstr::CStrPtr;
pub use crate::pattern::option::{Option, Option::None, Option::Some};
pub use crate::pattern::primitive::{Bool, CChar};
pub use crate::pattern::result::{Result, Result::Err, Result::Ok};
pub use crate::pattern::slice::{Slice, SliceMut};
pub use crate::pattern::string::String;
pub use crate::pattern::vec::Vec;

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
