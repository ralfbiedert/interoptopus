use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus::inventory::InventoryBuilder;
use interoptopus::pattern::slice::Slice;
use interoptopus::{callback, ffi_function, function};
use interoptopus_backend_csharp::InteropBuilder;
use tests::backend_csharp::common_namespace_mappings;

callback!(Foo(slice: Slice<u8>) -> u8);

/// Has documentation
#[ffi_function]
fn f(_: Foo) {}

#[test]
fn can_produce_markdown() -> Result<(), Error> {
    let inventory = InventoryBuilder::new().register(function!(f)).validate().build();
    let _ = InteropBuilder::new()
        .debug(true)
        .inventory(inventory)
        .namespace_mappings(common_namespace_mappings())
        .build()?
        .to_string();

    Ok(())
}
