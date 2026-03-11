use crate::Interop;
use interoptopus_backend_utils::{Error, IndentWriter, render};

pub fn write_imports(_i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    render!(w, "imports.py")
}
