use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_namespace_context(i: &Interop, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
    i.debug(w, "write_namespace_context")?;
    indented!(w, r"namespace {}", i.namespace_for_id(&i.namespace_id))?;
    indented!(w, r"{{")?;
    w.indent();

    f(w)?;

    w.unindent();

    indented!(w, r"}}")
}
