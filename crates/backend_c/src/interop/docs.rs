use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::Documentation;
use interoptopus::{Error, indented};
use std::io::BufRead;

pub fn write_file_header_comments(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, "{}", i.file_header_comment)
}

pub fn write_documentation(w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
    for line in documentation.lines() {
        indented!(w, r"/// {}", line)?;
    }

    Ok(())
}
