use crate::{Interop, render};
use interoptopus::Error;
use interoptopus::backend::IndentWriter;

pub fn write_utf8_string(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        render!(w, "builtins/utf8string.cs")?;
    }
    Ok(())
}
