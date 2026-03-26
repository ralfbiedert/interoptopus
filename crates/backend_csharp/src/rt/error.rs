//! Error type for .NET runtime initialization failures.

use std::fmt;

/// An error that occurred when initializing the .NET runtime.
#[derive(Debug)]
pub struct RuntimeError(String);

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for RuntimeError {}

impl From<std::io::Error> for RuntimeError {
    fn from(e: std::io::Error) -> Self {
        Self(format!("IO error: {e}"))
    }
}

impl From<netcorehost::nethost::LoadHostfxrError> for RuntimeError {
    fn from(e: netcorehost::nethost::LoadHostfxrError) -> Self {
        Self(format!("failed to load hostfxr: {e}"))
    }
}

impl From<netcorehost::error::HostingError> for RuntimeError {
    fn from(e: netcorehost::error::HostingError) -> Self {
        Self(format!("failed to initialize .NET runtime: {e}"))
    }
}

impl From<String> for RuntimeError {
    fn from(s: String) -> Self {
        Self(s)
    }
}
