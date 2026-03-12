use interoptopus::ffi;
use interoptopus::function;

#[ffi]
pub fn my_function() {}

#[test]
fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
    let plugin = debug_plugin!(|_inventory, models| {
        dbg!(&models);
    });

    test_ffi!(plugin, [function!(my_function)])?;

    Ok(())
}
