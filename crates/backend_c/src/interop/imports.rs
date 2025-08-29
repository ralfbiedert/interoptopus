use crate::Interop;
use interoptopus_backend_utils::{Error, IndentWriter, indented};

pub fn write_imports(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"#include <stdint.h>")?;
    indented!(w, r"#include <stdbool.h>")?;
    indented!(w, r"#include <sys/types.h>")?;

    // Write any user supplied includes into the file.
    for include in &i.additional_includes {
        indented!(w, "#include {}", include)?;
    }

    Ok(())
}
