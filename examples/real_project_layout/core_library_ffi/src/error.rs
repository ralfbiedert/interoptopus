// This file just implements the "error" pattern so that services
// work. Have a look at the documentation.

use interoptopus::ffi_type;
use std::fmt::{Display, Formatter};

#[ffi_type]
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum Error {
    Fail,
}

impl From<interoptopus::Error> for Error {
    fn from(_: interoptopus::Error) -> Self {
        Self::Fail
    }
}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}
