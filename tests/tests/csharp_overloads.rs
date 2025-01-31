use anyhow::Error;
use interoptopus::Generate;
use interoptopus_backend_csharp::{ConfigBuilder, Generator};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn dotnet() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().namespace_mappings(common_namespace_mappings()).build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "csharp_overloads_dotnet.cs", generated.as_str());

    Ok(())
}
