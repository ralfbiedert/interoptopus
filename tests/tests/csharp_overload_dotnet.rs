use anyhow::Error;
use interoptopus::Interop;
use interoptopus_backend_csharp::overloads::DotNet;
use interoptopus_backend_csharp::{ConfigBuilder, Generator};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn basic() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let overload = DotNet::new();
    let config = ConfigBuilder::default().namespace_mappings(common_namespace_mappings()).build()?;
    let generated = Generator::new(config, inventory).add_overload_writer(overload).write_string()?;

    validate_output!("tests", "csharp_overload_dotnet.cs", generated.as_str());

    Ok(())
}
