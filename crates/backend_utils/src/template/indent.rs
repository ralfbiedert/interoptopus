use std::error::Error;
use std::fmt::{Display, Formatter, Pointer, Write};

pub enum CurlyPlacement {
    Newline,
    EndOfLine,
}

pub struct IndentConfig {
    pub indent: String,
    pub curly: CurlyPlacement,
}

impl Default for IndentConfig {
    fn default() -> Self {
        Self { indent: "    ".to_string(), curly: CurlyPlacement::Newline }
    }
}

pub fn reindent(src: impl AsRef<str>, config: &IndentConfig) -> Result<String, ReindentError> {
    let mut out = String::with_capacity(src.as_ref().len() * 2);

    // TODO
    write!(out, "TODO");

    Ok(out)
}

#[derive(Debug)]
pub struct ReindentError {
    msg: String,
}

impl Error for ReindentError {}

impl Display for ReindentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
