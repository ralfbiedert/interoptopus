use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, render};

pub fn write_async_helper(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        render!(w, "builtins/async_helper.cs")?;
    }
    Ok(())
}
