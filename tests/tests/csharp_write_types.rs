use anyhow::Error;
use interoptopus::Bindings;
use interoptopus_backend_csharp::{ConfigBuilder, Generator, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn all() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .write_types(WriteTypes::All)
        .build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "csharp_write_types_all.cs", generated.as_str());

    Ok(())
}

#[test]
fn namespace() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .write_types(WriteTypes::Namespace)
        .build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "csharp_write_types_namespace.cs", generated.as_str());

    Ok(())
}

#[test]
fn namespace_and_global() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "csharp_write_types_namespace_and_global.cs", generated.as_str());

    Ok(())
}
