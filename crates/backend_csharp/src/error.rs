use std::fmt::{Display, Formatter};

/// An error that occurred during C# code generation.
#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl From<interoptopus_backends::Error> for Error {
    fn from(e: interoptopus_backends::Error) -> Self {
        Self(format!("Template error: {e}"))
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
