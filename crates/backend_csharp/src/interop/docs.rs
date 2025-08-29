use crate::Interop;
use interoptopus::lang::Docs;
use interoptopus::pattern::api_guard::ApiHash;
use interoptopus_backend_utils::{Error, IndentWriter, indented, render};

const INTEROPTOPUS_CRATE: &str = env!("CARGO_PKG_NAME");
const INTEROPTOPUS_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn write_file_header_comments(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let hash = ApiHash::from(&i.inventory);
    let hash_hex = hash.hash_hex();

    if let Some(header) = &i.file_header_comment {
        let mut context = tera::Context::new();
        context.insert("INTEROP_HASH", hash_hex);
        context.insert("INTEROP_DLL_NAME", &i.dll_name);
        context.insert("INTEROP_NAMESPACE", &i.namespace_id);
        context.insert("INTEROPTOPUS_CRATE", INTEROPTOPUS_CRATE);
        context.insert("INTEROPTOPUS_VERSION", INTEROPTOPUS_VERSION);
        let rendered = tera::Tera::one_off(header, &context, true).map_err(Error::Templating)?;
        indented!(w, "{}", rendered)?;
        Ok(())
    } else {
        render!(
            w,
            "file_header.cs",
            ("INTEROP_HASH", hash_hex),
            ("INTEROP_DLL_NAME", &i.dll_name),
            ("INTEROP_NAMESPACE", &i.namespace_id),
            ("INTEROPTOPUS_CRATE", INTEROPTOPUS_CRATE),
            ("INTEROPTOPUS_VERSION", INTEROPTOPUS_VERSION)
        )
    }
}

pub fn write_documentation(w: &mut IndentWriter, documentation: &Docs) -> Result<(), Error> {
    for line in documentation.lines() {
        indented!(w, r"///{}", line)?;
    }

    Ok(())
}
