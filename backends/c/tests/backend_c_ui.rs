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

    let inventory = interoptopus_reference_project::ffi_inventory();

    let generator = Generator::new(config, inventory);
    generator.write_file(format!("{}/my_header.h", output))?;

    // let doc_gen = DocGenerator::new(library, generator);
    // doc_gen.write_file(format!("{}/my_header.md", output))?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_match_reference() -> Result<(), Error> {
    generate_bindings("tests/output/")?;

    assert_file_matches_generated("tests/output/my_header.h");

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_work() -> Result<(), Error> {
    generate_bindings("tests/output/")?;

    compile_c_app_if_installed("tests/output/", "tests/output/app.c")?;
    Ok(())
}
