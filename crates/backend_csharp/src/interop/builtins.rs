use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_builtins(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() && i.has_ffi_error(i.inventory.functions()) {
        let error_text = &i.error_text;

        indented!(w, r"public class InteropException<T> : Exception")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"public T Error {{ get; private set; }}")?;
        w.newline()?;
        indented!(w, [()], r#"public InteropException(T error): base($"{error_text}")"#)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"Error = error;")?;
        indented!(w, [()], r"}}")?;
        indented!(w, r"}}")?;
        w.newline()?;
    }

    Ok(())
}
