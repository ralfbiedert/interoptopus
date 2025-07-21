use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::Docs;
use interoptopus::pattern::api_guard::ApiHash;
use interoptopus::{Error, indented};

const INTEROPTOPUS_CRATE: &str = env!("CARGO_PKG_NAME");
const INTEROPTOPUS_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn write_file_header_comments(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let hash = ApiHash::from(&i.inventory);
    let hash_hex = hash.hash_hex();

    let header = &i.file_header_comment;
    let header = header.replace("{INTEROP_HASH}", hash_hex);
    let header = header.replace("{INTEROP_DLL_NAME}", &i.dll_name);
    let header = header.replace("{INTEROP_NAMESPACE}", &i.namespace_id);
    let header = header.replace("{INTEROPTOPUS_CRATE}", INTEROPTOPUS_CRATE);
    let header = header.replace("{INTEROPTOPUS_VERSION}", INTEROPTOPUS_VERSION);

    indented!(w, r"{}", header)?;
    Ok(())
}

pub fn write_documentation(w: &mut IndentWriter, documentation: &Docs) -> Result<(), Error> {
    for line in documentation.lines() {
        indented!(w, r"///{}", line)?;
    }

    Ok(())
}
