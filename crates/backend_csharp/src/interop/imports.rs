use crate::Interop;
use interoptopus_backend_utils::{Error, IndentWriter, render};

pub fn write_imports(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_imports")?;

    let is_wired = i.has_emittable_wired_types();

    let mut namespace_imports = i.namespace_mappings.iter().map(|x| x.1.as_str().to_string()).collect::<Vec<_>>();
    namespace_imports.sort();
    namespace_imports.dedup();

    render!(w, "imports.cs", ("namespace_imports", &namespace_imports), ("is_wired", &is_wired))
}
