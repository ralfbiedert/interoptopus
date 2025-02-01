use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_file_header_comments(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"{}", &i.file_header_comment)?;
    Ok(())
}
