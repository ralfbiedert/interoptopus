use crate::Interop;
use interoptopus_backend_utils::{Error, IndentWriter, indented};

pub fn write_class_context(i: &Interop, class_name: &str, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
    i.debug(w, "write_class_context")?;
    indented!(w, r"{} static partial class {}", i.visibility_types.to_access_modifier(), class_name)?;
    indented!(w, r"{{")?;
    w.indent();

    f(w)?;

    w.unindent();
    indented!(w, r"}}")?;
    Ok(())
}

pub fn write_native_lib_string(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_native_lib_string")?;
    indented!(w, r#"public const string NativeLib = "{}";"#, i.dll_name)?;
    Ok(())
}
