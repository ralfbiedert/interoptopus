use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_imports(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_imports")?;

    indented!(w, r"#pragma warning disable 0105")?;
    indented!(w, r"using System;")?;
    indented!(w, r"using System.Text;")?;
    indented!(w, r"using System.Threading.Tasks;")?;
    indented!(w, r"using System.Collections;")?;
    indented!(w, r"using System.Collections.Generic;")?;
    indented!(w, r"using System.Runtime.InteropServices;")?;
    indented!(w, r"using System.Runtime.InteropServices.Marshalling;")?;
    indented!(w, r"using System.Runtime.CompilerServices;")?;

    for (_, v) in &i.namespace_mappings {
        indented!(w, r"using {v};")?;
    }

    indented!(w, r"#pragma warning restore 0105")?;

    Ok(())
}
