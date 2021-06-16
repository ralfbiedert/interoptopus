use interoptopus::generators::Interop;
use interoptopus::testing::c::compile_c_app_if_installed;
use interoptopus::Error;
use std::fs::read_to_string;

fn generate_bindings(output: &str) -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator};

    Generator::new(Config::default(), interoptopus_reference_project::ffi_inventory()).write_file(output)
}

#[test]
fn bindings_match_reference() -> Result<(), Error> {
    generate_bindings("tests/output/my_header.h")?;

    let actual = read_to_string("tests/output/my_header.h")?;
    let expected = read_to_string("tests/output/my_header.h.expected")?;

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn bindings_work() -> Result<(), Error> {
    generate_bindings("tests/output/my_header.h")?;
    compile_c_app_if_installed("tests/output/", "tests/output/app.c")?;
    Ok(())
}
