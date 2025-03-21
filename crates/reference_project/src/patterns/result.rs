use interoptopus::pattern::result::Result;
use interoptopus::{ffi_function, ffi_type};
use std::fmt::Display;
// This file may look complex but the Interoptopus parts are actually really simple,
// with some Rust best practices making up most of the code.

// This is the FFI error enum you want your users to see. You are free to name and implement this
// almost any way you want.
#[ffi_type]
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Error {
    Ok = 0,
    Null = 100,
    Panic = 200,
    Delegate = 300,
    Fail = 400,
}

// Implement Default so we know what the "good" case is.
impl Default for Error {
    fn default() -> Self {
        Self::Ok
    }
}

// Implement Interoptopus' `FFIError` trait for your FFIError enum.
// Here you must map 3 "well known" variants to your enum.
impl interoptopus::pattern::result::FFIError for Error {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::Null;
    const PANIC: Self = Self::Panic;
}

#[ffi_function]
pub fn pattern_result_1(x: Result<u32, Error>) -> Result<u32, Error> {
    x
}

#[ffi_function]
pub fn pattern_result_2() -> Result<(), Error> {
    Result::Ok(())
}

#[ffi_function]
pub fn pattern_result_3(x: Result<(), Error>) -> Result<(), Error> {
    x
}
