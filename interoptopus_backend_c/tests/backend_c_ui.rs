use interoptopus::writer::IndentWriter;
use interoptopus::Error;
use std::fs::{read_to_string, File};
use interoptopus::testing::c::compile_c_app_if_installed;

fn generate_bindings(output: &str) -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator, Interop};

    let library = interoptopus_reference_project::ffi_inventory();
    let config = Config {..Config::default() };
    let generator = Generator::new(config, library);

    let mut file = File::create(output)?;
    let mut writer = IndentWriter::new(&mut file);

    generator.write_to(&mut writer)?;

    Ok(())
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
