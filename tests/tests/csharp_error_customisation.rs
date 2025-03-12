use anyhow::Error;
use interoptopus::inventory::{Bindings, InventoryBuilder};
use interoptopus::{ffi_function, ffi_type, function};
use interoptopus_backend_csharp::{InteropBuilder, WriteTypes};
use tests::backend_csharp::common_namespace_mappings;

#[ffi_type(error)]
#[derive(Debug, PartialEq, Eq)]
enum FFIError {
    Success,
    Null,
    Panic,
}

impl interoptopus::pattern::result::FFIError for FFIError {
    const SUCCESS: Self = Self::Success;
    const NULL: Self = Self::Null;
    const PANIC: Self = Self::Panic;
}

#[ffi_function]
fn sample_function() -> FFIError {
    FFIError::Success
}

#[test]
fn enabled() -> Result<(), Error> {
    let inventory = InventoryBuilder::new().register(function!(sample_function)).build();

    let generated = InteropBuilder::new()
        .inventory(inventory)
        .namespace_mappings(common_namespace_mappings())
        .error_text("MY ERROR TEXT {}".to_string())
        .write_types(WriteTypes::All)
        .build()?
        .to_string()?;

    assert!(generated.contains("MY ERROR TEXT"));

    Ok(())
}
