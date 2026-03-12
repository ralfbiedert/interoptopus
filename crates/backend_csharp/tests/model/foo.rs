use interoptopus::ffi;
use interoptopus::inventory::RustInventory;
use interoptopus_csharp::debug_plugin;

#[ffi]
pub fn my_function() {}

// We just trick a unit test into producing our bindings, here for C#
#[test]
#[rustfmt::skip]
fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
    use interoptopus::function;
    use interoptopus::inventory::Inventory;
    use interoptopus_csharp::RustLibrary;

    let plugin = debug_plugin!(|_inventory, models| {
        dbg!(&models);
    });

    let inventory = RustInventory::new()
        .register(function!(my_function))
        .validate();

    RustLibrary::builder(inventory)
        .build()
        .register_plugin(plugin)
        .process()?;

    Ok(())
}
