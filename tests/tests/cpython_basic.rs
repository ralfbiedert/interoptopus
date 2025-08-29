use anyhow::Error;
use interoptopus_backend_cpython::Interop;
use interoptopus_reference_project::ffi_inventory;
use tests::backend_cpython::run_python_if_installed;
use tests::validate_output;

#[test]
fn bindings_work() -> Result<(), Error> {
    let generated = Interop::builder().inventory(ffi_inventory()).build()?.to_string()?;

    validate_output!("tests", "cpython_basic.py", generated.as_str());
    run_python_if_installed("tests", "cpython_basic.py")?;

    Ok(())
}
