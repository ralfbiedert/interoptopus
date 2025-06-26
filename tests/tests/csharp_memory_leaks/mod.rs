use anyhow::Error;
use interoptopus::backend::NAMESPACE_COMMON;
use interoptopus::inventory::Bindings;
use interoptopus_backend_csharp::{Interop, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn memory_leaks_prerequisites() -> Result<(), Error> {
    let generated_common = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_id(NAMESPACE_COMMON)
        .namespace_mappings(common_namespace_mappings())
        .dll_name("interoptopus_reference_project")
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .debug(false)
        .build()?
        .to_string()?;

    let generated_other = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .dll_name("interoptopus_reference_project")
        .write_types(WriteTypes::Namespace)
        .debug(false)
        .build()?
        .to_string()?;

    validate_output!("tests/csharp_reference_project", "Interop.common.cs", generated_common.as_str());
    validate_output!("tests/csharp_reference_project", "Interop.cs", generated_other.as_str());

    // Test is currently Windows specific w.r.t. memory usage metrics.
    // run_dotnet_command_if_installed("tests/csharp_memory_leaks", "test")?;

    Ok(())
}
