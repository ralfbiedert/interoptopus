use anyhow::Error;
use interoptopus::Interop;
use interoptopus_backend_c::{CDocumentationStyle, ConfigBuilder, Generator};
use interoptopus_reference_project::ffi_inventory;
use tests::{compile_output, validate_output};

#[test]
fn inline() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().documentation(CDocumentationStyle::Inline).build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests", "c_docs_inline.h", generated.as_str());
    compile_output!(generated.as_str());

    Ok(())
}

#[test]
fn none() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().documentation(CDocumentationStyle::None).build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests", "c_docs_none.h", generated.as_str());
    compile_output!(generated.as_str());

    Ok(())
}
