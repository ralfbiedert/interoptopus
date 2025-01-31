use anyhow::Error;
use interoptopus::Bindings;
use interoptopus_backend_cpython::{ConfigBuilder, Generator};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_cpython::run_python_if_installed;
use tests::validate_output;

#[test]
fn bindings_work() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "cpython_basic.py", generated.as_str());
    run_python_if_installed("tests", "cpython_basic.py")?;

    Ok(())
}
