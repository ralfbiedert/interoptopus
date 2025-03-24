use interoptopus::pattern::result::Result;
use interoptopus::{ffi_function, ffi_type};
// This file may look complex but the Interoptopus parts are actually really simple,
// with some Rust best practices making up most of the code.

// This is the FFI error enum you want your users to see. You are free to name and implement this
// almost any way you want.
#[ffi_type]
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Error {
    Fail,
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
