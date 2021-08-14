use interoptopus::testing::assert_file_matches_generated;
use interoptopus::Error;
use interoptopus::Interop;
use interoptopus_backend_c::compile_c_app_if_installed;

fn generate_bindings(output: &str) -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator};

    let config = Config {
        prefix: "my_library_".to_string(),
        ..Config::default()
    };

    Generator::new(config, interoptopus_reference_project::ffi_inventory()).write_file(output)
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_match_reference() -> Result<(), Error> {
    generate_bindings("tests/output/my_header.h")?;

    assert_file_matches_generated("tests/output/my_header.h");

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_work() -> Result<(), Error> {
    generate_bindings("tests/output/my_header.h")?;
    compile_c_app_if_installed("tests/output/", "tests/output/app.c")?;
    Ok(())
}
