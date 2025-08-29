use crate::Interop;
use interoptopus_backend_utils::{Error, IndentWriter, render};

pub fn write_interop_exception(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        render!(w, "builtins/exception.cs")?;
    }
    Ok(())
}
