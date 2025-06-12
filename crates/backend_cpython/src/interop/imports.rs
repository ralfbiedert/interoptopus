use crate::{Interop, render};
use interoptopus::Error;
use interoptopus::backend::IndentWriter;

pub fn write_imports(_i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    render!(w, "imports.py")
}
