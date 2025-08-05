use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus_backend_csharp::{Interop, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn all() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .write_types(WriteTypes::All)
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_write_types_all.cs", generated.as_str());

    Ok(())
}

#[test]
fn namespace() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .write_types(WriteTypes::Namespace)
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_write_types_namespace.cs", generated.as_str());

    Ok(())
}

#[test]
fn namespace_and_global() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .decorate_fn(|_| "[DefaultDllImportSearchPaths(DllImportSearchPath.AssemblyDirectory)]".to_string())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_write_types_namespace_and_global.cs", generated.as_str());

    Ok(())
}
