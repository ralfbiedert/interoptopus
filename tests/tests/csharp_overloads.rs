use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus_backend_csharp::Interop;
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn dotnet() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_overloads_dotnet.cs", generated.as_str());

    Ok(())
}
