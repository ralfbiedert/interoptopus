use interoptopus::ffi_type;
use std::fmt::{Display, Formatter};

#[ffi_type(patterns(ffi_error))]
#[repr(C)]
pub enum FFIError {
    Ok = 0,
    Null = 100,
    Panic = 200,
    Fail = 300,
}

impl Default for FFIError {
    fn default() -> Self {
        Self::Ok
    }
}

impl interoptopus::patterns::result::FFIError for FFIError {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::Null;
    const PANIC: Self = Self::Panic;
}

// An error we use in a Rust library
#[derive(Debug)]
pub enum Error {
    Bad,
}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}

impl From<Error> for FFIError {
    fn from(x: Error) -> Self {
        match x {
            Error::Bad => Self::Fail,
        }
    }
}
