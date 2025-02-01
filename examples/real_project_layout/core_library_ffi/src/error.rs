// This file just implements the "error" pattern so that services
// work. Have a look at the documentation.

use interoptopus::ffi_type;
use std::fmt::{Display, Formatter};

#[ffi_type(error)]
#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum FFIError {
    Ok = 0,
    Null = 100,
    Panic = 200,
    Delegate = 300,
    Fail = 400,
}

#[derive(Debug)]
pub enum Error {
    Bad,
}

impl From<interoptopus::Error> for Error {
    fn from(_: interoptopus::Error) -> Self {
        Self::Bad
    }
}

impl From<Error> for FFIError {
    fn from(x: Error) -> Self {
        match x {
            Error::Bad => Self::Fail,
        }
    }
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

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}
