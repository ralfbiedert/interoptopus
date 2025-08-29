use anyhow::Error;
use interoptopus::inventory::Inventory;
use interoptopus::{ffi_function, function};
use interoptopus_backend_c::{Functions, Interop};
use tests::{compile_output_c, validate_output};

#[ffi_function]
fn sample_function() {}

fn ffi_inventory() -> Inventory {
    Inventory::builder().register(function!(sample_function)).build()
}

#[test]
fn forward() -> Result<(), Error> {
    let generated = Interop::builder()
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
    let generated = Interop::builder()
        .inventory(interoptopus_reference_project::ffi_inventory())
        .function_style(Functions::Typedefs)
        .build()?
        .to_string()?;

    validate_output!("tests", "c_function_styles_typedefs.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
