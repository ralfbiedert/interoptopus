use anyhow::Error;
use interoptopus::extra_type;
use interoptopus::inventory::Bindings;
use interoptopus::inventory::Inventory;
use interoptopus::{callback, ffi_type};
use interoptopus_backend_csharp::Interop;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[ffi_type]
#[allow(dead_code)]
enum MyEnum {
    Enumerator,
}

callback!(EnumArgument(arg: MyEnum));
callback!(EnumReturn() -> MyEnum);

fn ffi_inventory() -> Inventory {
    Inventory::builder().register(extra_type!(EnumArgument)).register(extra_type!(EnumReturn)).build()
}

#[test]
fn csharp_callback_with_enum() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_callback_with_enum.cs", generated.as_str());

    Ok(())
}
