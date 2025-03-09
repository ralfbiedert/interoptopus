use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_custom_defines(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, "{}", i.custom_defines)
}

pub fn write_ifndef(i: &Interop, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
    if i.directives {
        indented!(w, r"#ifndef {}", i.ifndef)?;
        indented!(w, r"#define {}", i.ifndef)?;
        w.newline()?;
    }

    f(w)?;

    if i.directives {
        w.newline()?;
        indented!(w, r"#endif /* {} */", i.ifndef)?;
    }

    Ok(())
}

pub fn write_ifdefcpp(i: &Interop, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
    if i.directives {
        indented!(w, r"#ifdef __cplusplus")?;
        indented!(w, r#"extern "C" {{"#)?;
        indented!(w, r"#endif")?;
        w.newline()?;
    }

    f(w)?;

    if i.directives {
        w.newline()?;
        indented!(w, r"#ifdef __cplusplus")?;
        indented!(w, r"}}")?;
        indented!(w, r"#endif")?;
    }
    Ok(())
}
