use std::fmt::{Display, Formatter};

/// Can be observed if something goes wrong.
#[derive(Debug)]
pub enum Error {
    /// Formatting a string failed.
    Format(std::fmt::Error),

    /// Writing output failed.
    IO(std::io::Error),
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self::Format(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl Display for Error {
    // TODO: This should be nicer.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interoptopus failed!")
    }
}

// TODO
impl std::error::Error for Error {}
