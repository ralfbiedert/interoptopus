//! Error type for .NET runtime initialization failures.

use std::fmt;

/// An error that occurred when initializing the .NET runtime.
#[derive(Debug)]
pub struct DotnetError(String);

impl fmt::Display for DotnetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for DotnetError {}

impl From<std::io::Error> for DotnetError {
    fn from(e: std::io::Error) -> Self {
        Self(format!("IO error: {e}"))
    }
}

impl From<netcorehost::nethost::LoadHostfxrError> for DotnetError {
    fn from(e: netcorehost::nethost::LoadHostfxrError) -> Self {
        Self(format!("failed to load hostfxr: {e}"))
    }
}

impl From<netcorehost::error::HostingError> for DotnetError {
    fn from(e: netcorehost::error::HostingError) -> Self {
        Self(format!("failed to initialize .NET runtime: {e}"))
    }
}

impl From<String> for DotnetError {
    fn from(s: String) -> Self {
        Self(s)
    }
}
