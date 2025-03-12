use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus_backend_c::{DocStyle, InteropBuilder};
use interoptopus_reference_project::ffi_inventory;
use tests::{compile_output_c, validate_output};

#[test]
fn inline() -> Result<(), Error> {
    let generated = InteropBuilder::new()
        .inventory(ffi_inventory())
        .documentation(DocStyle::Inline)
        .build()?
        .to_string()?;

    validate_output!("tests/c_reference_project", "reference_project.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
