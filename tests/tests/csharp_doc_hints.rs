use anyhow::Error;
use interoptopus::inventory::Inventory;
use interoptopus::pattern::slice::Slice;
use interoptopus::{ffi_function, function};
use interoptopus_backend_csharp::Interop;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[ffi_function]
fn sample_function(_: Slice<u8>) {}

fn ffi_inventory() -> Inventory {
    Inventory::builder().register(function!(sample_function)).build()
}

#[test]
fn doc_hints_on() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_doc_hints_on.cs", generated.as_str());

    Ok(())
}

#[test]
fn doc_hints_off() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_doc_hints_off.cs", generated.as_str());

    Ok(())
}
