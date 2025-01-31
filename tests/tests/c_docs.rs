use anyhow::Error;
use interoptopus::{ffi_function, function, Generate, Inventory, InventoryBuilder};
use interoptopus_backend_c::{CDocumentationStyle, ConfigBuilder, Generator};
use tests::{compile_output_c, validate_output};

/// Documented
#[ffi_function]
fn sample_function() {}

fn ffi_inventory() -> Inventory {
    InventoryBuilder::new().register(function!(sample_function)).inventory()
}

#[test]
fn inline() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().documentation(CDocumentationStyle::Inline).build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "c_docs_inline.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}

#[test]
fn none() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().documentation(CDocumentationStyle::None).build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "c_docs_none.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
