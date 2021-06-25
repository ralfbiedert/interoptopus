use interoptopus::testing::assert_file_matches_generated;
use interoptopus::testing::csharp::run_dotnet_command_if_installed;
use interoptopus::util::NamespaceMappings;
use interoptopus::Error;
use interoptopus::Interop;

fn generate_bindings_multi(prefix: &str) -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    let library = interoptopus_reference_project::ffi_inventory();
    let namespace_mappings = NamespaceMappings::new("My.Company").add("common", "My.Company.Common");

    let config = Config {
        dll_name: "interoptopus_reference_project".to_string(),
        namespace_mappings,
        emit_rust_visibility: true,
        ..Config::default()
    };

    for namespace_id in library.namespaces() {
        let file_name = format!("{}.{}.cs", prefix, namespace_id).replace("..", ".");

        let config = Config {
            namespace_id: namespace_id.clone(),
            ..config.clone()
        };

        Generator::new(config.clone(), interoptopus_reference_project::ffi_inventory()).write_file(file_name)?;
    }

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_match_reference() -> Result<(), Error> {
    generate_bindings_multi("tests/output/Interop")?;

    assert_file_matches_generated("tests/output/Interop.cs");
    assert_file_matches_generated("tests/output/Interop.common.cs");

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_work() -> Result<(), Error> {
    generate_bindings_multi("tests/output/Interop")?;

    run_dotnet_command_if_installed("tests/output/", "build")?;
    Ok(())
}
