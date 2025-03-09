use crate::Interop;
use interoptopus::lang::c::Documentation;
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_file_header_comments(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"{}", &i.file_header_comment)?;
    Ok(())
}

pub fn write_documentation(w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
    for line in documentation.lines() {
        indented!(w, r"///{}", line)?;
    }
    Ok(())
}
