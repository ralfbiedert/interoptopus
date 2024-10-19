use anyhow::Error;
use interoptopus::Interop;
use interoptopus_backend_cpython::{ConfigBuilder, Generator};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_cpython::run_python_if_installed;
use tests::validate_output;

#[test]
fn reference_tests_work() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests/cpython_reference_project", "reference_project.py", generated.as_str());

    let files = [
        "test_basic_loads_dll.py",
        "test_core_apis.py",
        "test_core_namespaces.py",
        "test_core_panics.py",
        "test_core_slices.py",
        "test_pattern_callbacks.py",
        "test_pattern_services.py",
        "test_pattern_strings.py",
    ];

    for file in files {
        run_python_if_installed("tests/cpython_reference_project", file)?;
    }

    Ok(())
}
