use std::fmt::{Display, Formatter};

/// Can be observed if something goes wrong.
#[derive(Debug)]
pub enum Error {
    /// A null pointer was observed where it wasn't expected.
    Null,

    /// Formatting a string failed.
    Format(std::fmt::Error),

    /// Writing output failed.
    IO(std::io::Error),

    /// Not valid UTF-8
    UTF8(std::str::Utf8Error),

    /// Not valid UTF-8
    FromUtf8(std::string::FromUtf8Error),

    /// A command to test was not found.
    CommandNotFound,

    /// A test failed to execute.
    TestFailed,
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self::Format(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::UTF8(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(e)
    }
}

impl Display for Error {
    // TODO: This should be nicer.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interoptopus failed!")
    }
}

// TODO
impl std::error::Error for Error {}
