use anyhow::Error;
use interoptopus::inventory::Inventory;
use interoptopus::{builtins_string, builtins_vec, ffi, ffi_function, function};
use interoptopus_backend_csharp::{Interop, WriteTypes};
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[ffi_function]
fn sample_function(_: ffi::Vec<u8>) {}

#[test]
fn decorates() -> Result<(), Error> {
    let inventory = Inventory::builder()
        .register(function!(sample_function))
        .register(builtins_string!())
        .register(builtins_vec!(u8))
        .build();

    let generated = Interop::builder()
        .inventory(inventory)
        .namespace_mappings(common_namespace_mappings())
        .write_types(WriteTypes::All)
        .decorate_fn(|_| "[DefaultDllImportSearchPaths(DllImportSearchPath.System32)]".to_string())
        .build()?
        .to_string()?;

    validate_output!("tests", "csharp_decorate_fn.cs", generated.as_str());

    Ok(())
}
