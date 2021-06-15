use interoptopus::writer::IndentWriter;
use interoptopus::Error;
use std::fs::{read_to_string, File};
use interoptopus::testing::csharp::run_dotnet_command_if_installed;

fn generate_bindings(output: &str) -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator, Interop};

    let library = interoptopus_reference_project::ffi_inventory();
    let config = Config {
        namespace: "My.Company".to_string(),
        class: "InteropClass".to_string(),
        dll_name: "example_complex".to_string(),
        ..Config::default()
    };

    let generator = Generator::new(config, library);

    let mut file = File::create(output)?;
    let mut writer = IndentWriter::new(&mut file);

    generator.write_to(&mut writer)?;

    Ok(())
}

#[test]
fn bindings_match_reference() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator, Interop};

    generate_bindings("tests/output/Interop.cs")?;

    let actual = read_to_string("tests/output/Interop.cs")?;
    let expected = read_to_string("tests/output/Interop.cs.expected")?;

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn bindings_work() -> Result<(), Error> {
    generate_bindings("tests/output/Interop.cs")?;
    run_dotnet_command_if_installed("tests/output/", "build")?;
    Ok(())
}
