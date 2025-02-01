use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_imports(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"#include <stdint.h>")?;
    indented!(w, r"#include <stdbool.h>")?;

    // Write any user supplied includes into the file.
    for include in &i.additional_includes {
        indented!(w, "#include {}", include)?;
    }

    Ok(())
}
