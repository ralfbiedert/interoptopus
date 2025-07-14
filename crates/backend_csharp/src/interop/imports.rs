use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_imports(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_imports")?;

    indented!(w, r"#pragma warning disable 0105")?;
    indented!(w, r"using System;")?;
    indented!(w, r"using System.Text;")?;
    indented!(w, r"using System.Threading.Tasks;")?;
    indented!(w, r"using System.Threading.Tasks.Sources;")?;
    indented!(w, r"using System.Collections;")?;
    indented!(w, r"using System.Collections.Concurrent;")?;
    indented!(w, r"using System.Collections.Generic;")?;
    indented!(w, r"using System.Runtime.InteropServices;")?;
    indented!(w, r"using System.Runtime.InteropServices.Marshalling;")?;
    indented!(w, r"using System.Runtime.CompilerServices;")?;

    let mut namespace_imports = i.namespace_mappings.iter().map(|x| x.1.as_str().to_string()).collect::<Vec<_>>();
    namespace_imports.sort();

    for v in namespace_imports {
        indented!(w, r"using {v};")?;
    }

    indented!(w, r"#pragma warning restore 0105")?;

    Ok(())
}
