use anyhow::Error;
use interoptopus::lang::NAMESPACE_COMMON;
use interoptopus_backend_csharp::{Interop, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn prerequisites() -> Result<(), Error> {
    let generated_common = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_id(NAMESPACE_COMMON.to_string())
        .namespace_mappings(common_namespace_mappings())
        .dll_name("interoptopus_reference_project".to_string())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?
        .to_string()?;

    let generated_other = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .dll_name("interoptopus_reference_project".to_string())
        .write_types(WriteTypes::Namespace)
        .build()?
        .to_string()?;

    validate_output!("tests/csharp_benchmarks", "Interop.common.cs", generated_common.as_str());
    validate_output!("tests/csharp_benchmarks", "Interop.cs", generated_other.as_str());

    Ok(())
}
