use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    AssetError(std::io::Error),
    AssetNotFound(String),
    AssetUtf8Error(String, std::string::FromUtf8Error),
    MissingOutDir,
    PathStripError,
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
