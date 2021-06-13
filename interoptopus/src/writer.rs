//! Types used by backends to produce pretty output.
use crate::Error;
use std::io::Write;

/// Convenience helper to allow backends to write code with indentation.
pub struct IndentWriter<'a> {
    one_indent: String,
    current_level: usize,
    writer: &'a mut dyn Write,
}

impl<'a> IndentWriter<'a> {
    pub fn indent(&mut self) {
        self.current_level += 1;
    }

    pub fn unindent(&mut self) {
        if self.current_level == 0 {
            panic!("Tried to un-indent past start of line.")
        }

        self.current_level -= 1;
    }

    pub fn new(writer: &'a mut dyn Write, one_indent: &str) -> Self {
        Self {
            one_indent: one_indent.to_string(),
            current_level: 0,
            writer,
        }
    }

    pub fn indented(&mut self, f: impl FnOnce(&mut dyn Write) -> std::io::Result<()>) -> Result<(), Error> {
        for _ in 0..self.current_level {
            write!(&mut self.writer, "{}", self.one_indent)?;
        }

        f(&mut self.writer)?;

        Ok(())
    }

    pub fn writer(&mut self) -> &mut dyn Write {
        &mut self.writer
    }

    pub fn newline(&mut self) -> Result<(), Error> {
        writeln!(&mut self.writer)?;
        Ok(())
    }
}
