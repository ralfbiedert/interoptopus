use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Todo,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interoptopus failed: {self:?}!")
    }
}

impl std::error::Error for Error {}
