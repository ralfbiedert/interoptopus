use interoptopus::ffi_type;
use std::fmt::{Display, Formatter};

// This file may look complex but the Interoptopus parts are actually really simple,
// with some Rust best practices making up most of the code.

// This is the FFI error enum you want your users to see. You are free to name and implement this
// almost any way you want.
#[ffi_type(patterns(ffi_error))]
#[repr(C)]
pub enum FFIError {
    Ok = 0,
    Null = 100,
    Panic = 200,
    Fail = 300,
}

// This is the error type you use in a Rust library. Again, you are almost
// entirely free how you want to implement it.
#[derive(Debug)]
pub enum Error {
    Bad,
}

// Provide a mapping how your Rust error enums translate
// to your FFI error enums.
impl From<Error> for FFIError {
    fn from(x: Error) -> Self {
        match x {
            Error::Bad => Self::Fail,
        }
    }
}

// Implement Default so we know what the "good" case is.
impl Default for FFIError {
    fn default() -> Self {
        Self::Ok
    }
}

// Implement Interoptopus' `FFIError` trait for your FFIError enum.
// Here you must map 3 "well known" variants to your enum.
impl interoptopus::patterns::result::FFIError for FFIError {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::Null;
    const PANIC: Self = Self::Panic;
}

// Lazy "Display" implementation so your error can be logged.
impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

// Tell Rust your error type is an actual Rust Error.
impl std::error::Error for Error {}
