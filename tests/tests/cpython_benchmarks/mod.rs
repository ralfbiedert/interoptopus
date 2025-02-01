use anyhow::Error;
use interoptopus::Bindings;
use interoptopus_backend_cpython::InteropBuilder;
use interoptopus_reference_project::ffi_inventory;
use tests::validate_output;

#[test]
fn reference_benchmarks_prerequisites() -> Result<(), Error> {
    let generated = InteropBuilder::default().inventory(ffi_inventory()).build()?.to_string()?;

    validate_output!("tests/cpython_benchmarks", "reference_project.py", generated.as_str());

    Ok(())
}
