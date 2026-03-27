use std::fmt::{Display, Formatter};

/// Can be observed if something goes wrong.
#[derive(Debug)]
pub struct Error(String);

impl Error {
    /// Create a new error with the given message.
    pub fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }

    /// A null pointer was observed where it wasn't expected.
    pub fn null() -> Self {
        Self("null pointer".into())
    }

    /// Given string is not nul terminated.
    pub fn nul_terminated() -> Self {
        Self("string is not nul terminated".into())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self(e.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}
