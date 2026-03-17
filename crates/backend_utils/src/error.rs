use std::fmt::{Display, Formatter};

/// Errors that can occur during backend code generation.
#[derive(Debug)]
pub enum Error {
    /// An I/O error while reading or writing assets.
    AssetError(std::io::Error),
    /// A requested asset path was not found in the archive.
    AssetNotFound(String),
    /// An asset file was not valid UTF-8.
    AssetUtf8Error(String, std::string::FromUtf8Error),
    /// The `OUT_DIR` environment variable is not set (expected in `build.rs`).
    MissingOutDir,
    /// Failed to strip a path prefix.
    PathStripError,
    /// A Tera template failed to render.
    TemplateRender(tera::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AssetError(e) => write!(f, "Asset I/O error: {e}"),
            Self::AssetNotFound(path) => write!(f, "Asset not found: {path}"),
            Self::AssetUtf8Error(path, e) => write!(f, "Asset '{path}' is not valid UTF-8: {e}"),
            Self::MissingOutDir => write!(f, "OUT_DIR environment variable not set (must be called from build.rs)"),
            Self::PathStripError => write!(f, "Failed to strip path prefix"),
            Self::TemplateRender(_) => write!(f, "Failed to render template"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::AssetError(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(_: std::env::VarError) -> Self {
        Self::MissingOutDir
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(_: std::path::StripPrefixError) -> Self {
        Self::PathStripError
    }
}
impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Self::TemplateRender(e)
    }
}
