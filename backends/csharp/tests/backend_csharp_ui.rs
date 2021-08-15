use interoptopus::testing::assert_file_matches_generated;
use interoptopus::util::NamespaceMappings;
use interoptopus::Error;
use interoptopus::Interop;
use interoptopus_backend_csharp::{run_dotnet_command_if_installed, Unsafe, WriteTypes};

fn generate_bindings_multi(prefix: &str, use_unsafe: Unsafe) -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    let library = interoptopus_reference_project::ffi_inventory();
    let namespace_mappings = NamespaceMappings::new("My.Company").add("common", "My.Company.Common");

    let config = Config {
        dll_name: "interoptopus_reference_project".to_string(),
        namespace_mappings,
        unroll_struct_arrays: true,
        emit_rust_visibility: true,
        use_unsafe,
        // debug: true,
        ..Config::default()
    };

    for namespace_id in library.namespaces() {
        let file_name = format!("{}.{}.cs", prefix, namespace_id).replace("..", ".");

        let write_types = if namespace_id.is_empty() {
            WriteTypes::Namespace
        } else {
            WriteTypes::NamespaceAndInteroptopusGlobal
        };

        let config = Config {
            namespace_id: namespace_id.clone(),
            write_types,
            ..config.clone()
        };

        Generator::new(config.clone(), interoptopus_reference_project::ffi_inventory()).write_file(file_name)?;
    }

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_match_reference() -> Result<(), Error> {
    generate_bindings_multi("tests/output_safe/Interop", Unsafe::None)?;
    generate_bindings_multi("tests/output_unsafe/Interop", Unsafe::UnsafePlatformMemCpy)?;

    assert_file_matches_generated("tests/output_safe/Interop.cs");
    assert_file_matches_generated("tests/output_safe/Interop.common.cs");

    assert_file_matches_generated("tests/output_unsafe/Interop.cs");
    assert_file_matches_generated("tests/output_unsafe/Interop.common.cs");

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_work() -> Result<(), Error> {
    generate_bindings_multi("tests/output_safe/Interop", Unsafe::None)?;
    generate_bindings_multi("tests/output_unsafe/Interop", Unsafe::UnsafePlatformMemCpy)?;

    run_dotnet_command_if_installed("tests/output_safe/", "test")?;
    run_dotnet_command_if_installed("tests/output_unsafe/", "test")?;
    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn prepare_benchmarks() -> Result<(), Error> {
    generate_bindings_multi("benches/Interop", Unsafe::UnsafePlatformMemCpy)?;
    Ok(())
}
