use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_imports(_i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"from __future__ import annotations")?;
    indented!(w, r"import ctypes")?;
    indented!(w, r"import typing")?;
    w.newline()?;
    indented!(w, r#"T = typing.TypeVar("T")"#)?;
    Ok(())
}
