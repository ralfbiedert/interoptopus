//! Error types for wire serialization / deserialization failures.

/// Returned when a wire serialize or deserialize operation fails.
#[derive(Debug)]
pub enum WireError {
    Io(std::io::Error),
    InvalidData(String),
    InvalidDiscriminant(String, usize),
}

impl From<std::io::Error> for WireError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
