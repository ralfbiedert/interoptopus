use interoptopus::testing::assert_file_matches_generated;
use interoptopus::{Error, Interop};
use interoptopus_backend_cpython::{run_python_if_installed, DocConfig, DocGenerator};

fn generate_bindings(output: &str) -> Result<(), Error> {
    use interoptopus_backend_cpython::{Config, Generator};

    Generator::new(Config::default(), interoptopus_reference_project::ffi_inventory()).write_file(output)
}

fn generate_documentation(output: &str) -> Result<(), Error> {
    use interoptopus_backend_cpython::{Config, Generator};

    let inventory = interoptopus_reference_project::ffi_inventory();
    let generator = Generator::new(Config::default(), interoptopus_reference_project::ffi_inventory());

    DocGenerator::new(&inventory, &generator, DocConfig::default()).write_file(output)
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_match_reference() -> Result<(), Error> {
    generate_bindings("tests/output/reference_project.py")?;
    generate_documentation("tests/output/reference_project.md")?;

    assert_file_matches_generated("tests/output/reference_project.py");

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_work() -> Result<(), Error> {
    generate_bindings("tests/output/reference_project.py")?;

    run_python_if_installed("tests/output/", "tests.py")?;

    Ok(())
}
