use std::fmt::{Display, Formatter};

/// Errors that can occur during C# code generation.
#[derive(Debug)]
pub enum Error {
    /// The model pass loop did not converge within the iteration limit.
    PassLimit,
    /// A Tera template failed to render.
    TemplateError(interoptopus_backends::Error),
    /// A type was referenced but has no C# name assigned.
    MissingTypeName(String),
    /// The `dotnet` CLI was found but a command it ran failed.
    DotNetCliCommandFailed(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TemplateError(e) => write!(f, "Template error: {e}"),
            Self::PassLimit => write!(f, "Pass iteration limit reached."),
            Self::MissingTypeName(ctx) => write!(f, "Missing type name: {ctx}"),
            Self::DotNetCliCommandFailed(ctx) => write!(f, "dotnet command failed: {ctx}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<interoptopus_backends::Error> for Error {
    fn from(e: interoptopus_backends::Error) -> Self {
        Self::TemplateError(e)
    }
}
