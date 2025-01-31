use anyhow::Error;
use interoptopus::{ffi_function, function, Bindings, Inventory, InventoryBuilder};
use interoptopus_backend_c::{ConfigBuilder, Functions, Generator};
use tests::{compile_output_c, validate_output};

#[ffi_function]
fn sample_function() {}

fn ffi_inventory() -> Inventory {
    InventoryBuilder::new().register(function!(sample_function)).inventory()
}

#[test]
fn forward() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().function_style(Functions::ForwardDeclarations).build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "c_function_styles_forward.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}

#[test]
fn typedef() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().function_style(Functions::Typedefs).build()?;
    let generated = Generator::new(config, inventory).to_string()?;

    validate_output!("tests", "c_function_styles_typedefs.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
