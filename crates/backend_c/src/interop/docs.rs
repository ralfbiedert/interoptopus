use crate::Interop;
use interoptopus::lang::c;
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_file_header_comments(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, "{}", i.file_header_comment)
}

pub fn write_documentation(w: &mut IndentWriter, documentation: &c::Documentation) -> Result<(), Error> {
    for line in documentation.lines() {
        indented!(w, r"/// {}", line)?;
    }

    Ok(())
}
