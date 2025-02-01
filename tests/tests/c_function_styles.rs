use anyhow::Error;
use interoptopus::Bindings;
use interoptopus::{ffi_function, function, Inventory, InventoryBuilder};
use interoptopus_backend_c::{Functions, InteropBuilder};
use tests::{compile_output_c, validate_output};

#[ffi_function]
fn sample_function() {}

fn ffi_inventory() -> Inventory {
    InventoryBuilder::new().register(function!(sample_function)).inventory()
}

#[test]
fn forward() -> Result<(), Error> {
    let generated = InteropBuilder::default()
        .inventory(ffi_inventory())
        .function_style(Functions::ForwardDeclarations)
        .build()?
        .to_string()?;

    validate_output!("tests", "c_function_styles_forward.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}

#[test]
fn typedef() -> Result<(), Error> {
    let generated = InteropBuilder::default()
        .inventory(interoptopus_reference_project::ffi_inventory())
        .function_style(Functions::Typedefs)
        .build()?
        .to_string()?;

    validate_output!("tests", "c_function_styles_typedefs.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
