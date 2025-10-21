use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    TemplateError(interoptopus_backends::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TemplateError(e) => write!(f, "Template error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<interoptopus_backends::Error> for Error {
    fn from(e: interoptopus_backends::Error) -> Self {
        Error::TemplateError(e)
    }
}
