use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, render};

pub fn write_utf8_string(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        render!(w, "builtins/utf8string.cs")?;
    }
    Ok(())
}
