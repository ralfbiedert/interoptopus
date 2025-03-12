// This file just implements the "error" pattern so that services
// work. Have a look at the documentation.

use interoptopus::ffi_type;
use std::fmt::{Display, Formatter};

#[ffi_type(error)]
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum Error {
    Ok = 0,
    Null = 100,
    Panic = 200,
    Delegate = 300,
    Fail = 400,
}

impl From<interoptopus::Error> for Error {
    fn from(_: interoptopus::Error) -> Self {
        Self::Fail
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::Ok
    }
}

impl interoptopus::pattern::result::FFIError for Error {
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
