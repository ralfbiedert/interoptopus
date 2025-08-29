/// If a wire transfer goes wrong.
// @todo play with implementing it as a struct?
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
