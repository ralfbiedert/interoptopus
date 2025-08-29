use anyhow::Error;
use interoptopus::inventory::Inventory;
use interoptopus::{api_guard, ffi_function, function};
use interoptopus_backend_csharp::Interop;
use tests::validate_output;

#[ffi_function]
fn sample_function() {}

fn ffi_inventory() -> Inventory {
    Inventory::builder()
        .register(api_guard!(ffi_inventory))
        .register(function!(sample_function))
        .validate()
        .build()
}

#[test]
fn csharp_api_guard() -> Result<(), Error> {
    let generated = Interop::builder().inventory(ffi_inventory()).build()?.to_string()?;

    validate_output!("tests", "csharp_api_guard.cs", generated.as_str());

    Ok(())
}
