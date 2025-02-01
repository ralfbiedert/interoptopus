use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_imports(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_imports")?;

    indented!(w, r"#pragma warning disable 0105")?;
    indented!(w, r"using System;")?;
    indented!(w, r"using System.Text;")?;
    indented!(w, r"using System.Collections;")?;
    indented!(w, r"using System.Collections.Generic;")?;
    indented!(w, r"using System.Runtime.InteropServices;")?;
    indented!(w, r"using System.Runtime.InteropServices.Marshalling;")?;
    indented!(w, r"using System.Runtime.CompilerServices;")?;

    for namespace_id in i.inventory.namespaces() {
        let namespace = i
            .namespace_mappings
            .get(namespace_id)
            .unwrap_or_else(|| panic!("Must have namespace for '{namespace_id}' ID"));

        indented!(w, r"using {};", namespace)?;
    }
    indented!(w, r"#pragma warning restore 0105")?;

    Ok(())
}
