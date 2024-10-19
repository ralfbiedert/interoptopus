use anyhow::Error;
use interoptopus::{ffi_function, function, Interop, Inventory, InventoryBuilder};
use interoptopus_backend_csharp::{ConfigBuilder, Generator, WriteTypes};
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[ffi_function]
fn sample_function() {}

fn ffi_inventory() -> Inventory {
    InventoryBuilder::new().register(function!(sample_function)).inventory()
}

#[test]
fn enabled() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .error_text("MY ERROR TEXT {}".to_string())
        .write_types(WriteTypes::All)
        .build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests", "csharp_error_customisation.cs", generated.as_str());

    Ok(())
}
