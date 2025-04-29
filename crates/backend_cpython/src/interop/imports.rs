use crate::Interop;
use interoptopus::{Error, backend::IndentWriter, render};

pub fn write_imports(_i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    render!(w, "imports.py")
}
