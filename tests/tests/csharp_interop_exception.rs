use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus::inventory::InventoryBuilder;
use interoptopus::{ffi_function, ffi_type, function};
use interoptopus_backend_csharp::{InteropBuilder, WriteTypes};

#[ffi_type(error)]
#[derive(Debug, PartialEq, Eq)]
enum FFIError {
    Success,
    Null,
    Panic,
}

impl interoptopus::patterns::result::FFIError for FFIError {
    const SUCCESS: Self = Self::Success;
    const NULL: Self = Self::Null;
    const PANIC: Self = Self::Panic;
}

#[ffi_function]
fn return_error() -> FFIError {
    FFIError::Success
}

#[ffi_function]
fn doesnt_return_error() {}

#[test]
fn has_exception() -> Result<(), Error> {
    let inventory = InventoryBuilder::new().register(function!(return_error)).build();
    let generated = InteropBuilder::new().inventory(inventory).write_types(WriteTypes::All).build()?.to_string()?;

    assert!(generated.contains("InteropException"));

    Ok(())
}

#[test]
fn no_exception() -> Result<(), Error> {
    let inventory = InventoryBuilder::new().register(function!(doesnt_return_error)).build();
    let generated = InteropBuilder::new().inventory(inventory).write_types(WriteTypes::All).build()?.to_string()?;

    assert!(!generated.contains("InteropException"));

    Ok(())
}
