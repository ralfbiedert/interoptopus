use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, render};

pub fn write_utf8_string(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        let class_name = i.class_constants.as_ref().map_or(&i.class, |name| name);
        let extra_fn_decorations = i.fn_decorations();
        render!(w, "builtins/utf8string.cs", ("class_name", class_name), ("extra_fn_decorations", &extra_fn_decorations))?;
    }
    Ok(())
}
