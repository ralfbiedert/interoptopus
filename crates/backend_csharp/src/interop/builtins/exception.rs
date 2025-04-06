use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_interop_exception(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        indented!(w, r"public class InteropException: Exception")?;
        indented!(w, r"{{")?;
        w.newline()?;
        indented!(w, [()], r"public InteropException(): base()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()], r"}}")?;
        indented!(w, r"}}")?;
        w.newline()?;
    }
    Ok(())
}
