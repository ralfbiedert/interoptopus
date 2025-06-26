use anyhow::Error;
use interoptopus::backend::NAMESPACE_COMMON;
use interoptopus::inventory::Bindings;
use interoptopus_backend_csharp::{Interop, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::{common_namespace_mappings, run_dotnet_command_if_installed};
use tests::validate_output;

const DEBUG: bool = false;

#[test]
fn prerequisites() -> Result<(), Error> {
    let generated_common = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_id(NAMESPACE_COMMON)
        .namespace_mappings(common_namespace_mappings())
        .dll_name("interoptopus_reference_project")
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .debug(DEBUG)
        .build()?
        .to_string()?;

    let generated_other = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .dll_name("interoptopus_reference_project")
        .write_types(WriteTypes::Namespace)
        .debug(DEBUG)
        .build()?
        .to_string()?;

    validate_output!("tests/csharp_reference_project", "Interop.common.cs", generated_common.as_str());
    validate_output!("tests/csharp_reference_project", "Interop.cs", generated_other.as_str());

    run_dotnet_command_if_installed("tests/csharp_reference_project", "test")?;

    Ok(())
}
