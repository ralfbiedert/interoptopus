use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus::inventory::Inventory;
use interoptopus::{builtins_string, ffi_function, function};
use interoptopus_backend_csharp::Interop;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[ffi_function]
fn sample_function() {}

fn ffi_inventory() -> Inventory {
    Inventory::builder().register(function!(sample_function)).register(builtins_string!()).build()
}

#[test]
fn classname_respected() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .class("MyClass")
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_class_name.cs", generated.as_str());

    Ok(())
}
