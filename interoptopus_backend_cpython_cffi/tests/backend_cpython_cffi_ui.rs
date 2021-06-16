use interoptopus::generators::Interop;
use interoptopus::testing::python::run_python_if_installed;
use interoptopus::Error;
use std::fs::read_to_string;

fn generate_bindings(output: &str) -> Result<(), Error> {
    use interoptopus_backend_cpython_cffi::{Config, Generator};

    Generator::new(Config::default(), interoptopus_reference_project::ffi_inventory()).write_file(output)
}

#[test]
fn bindings_match_reference() -> Result<(), Error> {
    generate_bindings("tests/output/reference_project.py")?;

    let actual = read_to_string("tests/output/reference_project.py")?;
    let expected = read_to_string("tests/output/reference_project.py.expected")?;

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn bindings_work() -> Result<(), Error> {
    generate_bindings("tests/output/Interop.cs")?;

    let output = run_python_if_installed("tests/output/", "app.py")?;

    dbg!(output);

    Ok(())
}
