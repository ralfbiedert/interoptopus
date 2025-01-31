use anyhow::Error;
use interoptopus::{ffi_function, ffi_type, function, Generate, InventoryBuilder};
use interoptopus_backend_csharp::{ConfigBuilder, Generator, WriteTypes};
use tests::backend_csharp::common_namespace_mappings;

#[ffi_type(error)]
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
fn sample_function() -> FFIError {
    FFIError::Success
}

#[test]
fn enabled() -> Result<(), Error> {
    let inventory = InventoryBuilder::new().register(function!(sample_function)).inventory();

    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .error_text("MY ERROR TEXT {}".to_string())
        .write_types(WriteTypes::All)
        .build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    assert!(generated.contains("MY ERROR TEXT"));

    Ok(())
}
