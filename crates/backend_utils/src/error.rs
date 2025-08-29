use std::fmt::{Display, Formatter};

/// Can be observed if something goes wrong.
#[derive(Debug)]
pub enum Error {
    /// A null pointer was observed where it wasn't expected.
    Null,

    /// An operation was requested that is not supported.
    Unsupported,

    /// Given string is not nul terminated.
    NulTerminated,

    /// Formatting a string failed.
    Format(std::fmt::Error),

    /// Writing output failed.
    Io(std::io::Error),

    /// Not valid UTF-8
    Utf8(std::str::Utf8Error),

    /// Not valid UTF-8
    FromUtf8(std::string::FromUtf8Error),

    /// Template generation failed.
    Templating(tera::Error),

    /// Interoptopus failed.
    Interoptopus(interoptopus::Error),

    /// A command to test was not found.
    CommandNotFound,

    /// A test failed to execute.
    TestFailed(String, String),

    /// A specified file was not found.
    FileNotFound,
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self::Format(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(e)
    }
}

impl From<interoptopus::Error> for Error {
    fn from(e: interoptopus::Error) -> Self {
        Self::Interoptopus(e)
    }
}

impl Display for Error {
    // TODO: This should be nicer.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interoptopus failed: {self:?}!")
    }
}

// TODO
impl std::error::Error for Error {}
