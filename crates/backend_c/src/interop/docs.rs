use crate::Interop;
use interoptopus::lang::Docs;
use interoptopus_backend_utils::{Error, IndentWriter, indented};

pub fn write_file_header_comments(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, "{}", i.file_header_comment)?;
    Ok(())
}

pub fn write_documentation(w: &mut IndentWriter, documentation: &Docs) -> Result<(), Error> {
    for line in documentation.lines() {
        indented!(w, r"///{}", line)?;
    }

    Ok(())
}
