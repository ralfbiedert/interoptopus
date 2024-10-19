use anyhow::Error;
use interoptopus::Interop;
use interoptopus_backend_csharp::overloads::DotNet;
use interoptopus_backend_csharp::{ConfigBuilder, Generator, Unsafe, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::{common_namespace_mappings, run_dotnet_command_if_installed};
use tests::validate_output;

#[test]
fn reference_benchmarks_prerequisites() -> Result<(), Error> {
    let config_common = ConfigBuilder::default()
        .namespace_id("common".to_string())
        .namespace_mappings(common_namespace_mappings())
        .use_unsafe(Unsafe::UnsafePlatformMemCpy)
        .dll_name("interoptopus_reference_project".to_string())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?;

    let config_other = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .use_unsafe(Unsafe::UnsafePlatformMemCpy)
        .dll_name("interoptopus_reference_project".to_string())
        .write_types(WriteTypes::Namespace)
        .build()?;

    let generated_common = Generator::new(config_common, ffi_inventory()).add_overload_writer(DotNet::new()).write_string()?;
    let generated_other = Generator::new(config_other, ffi_inventory()).add_overload_writer(DotNet::new()).write_string()?;

    validate_output!("tests/csharp_reference_project_unsafe", "Interop.common.cs", generated_common.as_str());
    validate_output!("tests/csharp_reference_project_unsafe", "Interop.cs", generated_other.as_str());

    run_dotnet_command_if_installed("tests/csharp_reference_project_unsafe", "test")?;

    Ok(())
}
