use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, render};

pub fn write_interop_exception(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        render!(w, "builtins/exception.cs")?;
    }
    Ok(())
}
