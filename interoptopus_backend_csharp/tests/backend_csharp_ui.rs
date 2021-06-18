use interoptopus::testing::csharp::run_dotnet_command_if_installed;
use interoptopus::Error;
use interoptopus::Interop;
use std::fs::read_to_string;

fn generate_bindings(output: &str) -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            namespace: "My.Company".to_string(),
            class: "InteropClass".to_string(),
            dll_name: "example_complex".to_string(),
            ..Config::default()
        },
        interoptopus_reference_project::ffi_inventory(),
    )
    .write_file(output)
}

#[test]
fn bindings_match_reference() -> Result<(), Error> {
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
