use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    AssetError(std::io::Error),
    AssetNotFound(String),
    AssetUtf8Error(String, std::string::FromUtf8Error),
    MissingOutDir,
    PathStripError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AssetError(e) => write!(f, "Asset I/O error: {}", e),
            Error::AssetNotFound(path) => write!(f, "Asset not found: {}", path),
            Error::AssetUtf8Error(path, e) => write!(f, "Asset '{}' is not valid UTF-8: {}", path, e),
            Error::MissingOutDir => write!(f, "OUT_DIR environment variable not set (must be called from build.rs)"),
            Error::PathStripError => write!(f, "Failed to strip path prefix"),
        }
    }
}

impl std::error::Error for Error {}
