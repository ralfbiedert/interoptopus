use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus::inventory::Inventory;
use interoptopus::pattern::slice::Slice;
use interoptopus::{ffi_function, function};
use interoptopus_backend_csharp::InteropBuilder;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[ffi_function]
fn sample_function(_: Slice<u8>) {}

fn ffi_inventory() -> Inventory {
    Inventory::builder().register(function!(sample_function)).build()
}

#[test]
fn spans_work() -> Result<(), Error> {
    let generated = InteropBuilder::new()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_slice_type.cs", generated.as_str());

    Ok(())
}
