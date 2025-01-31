use anyhow::Error;
use interoptopus::Bindings;
use interoptopus_backend_c::{ConfigBuilder, Documentation, Generator};
use interoptopus_reference_project::ffi_inventory;
use tests::{compile_output_c, validate_output};

#[test]
fn inline() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().documentation(Documentation::Inline).build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests/c_reference_project", "reference_project.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
