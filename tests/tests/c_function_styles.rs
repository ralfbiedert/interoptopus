use anyhow::Error;
use interoptopus::Interop;
use interoptopus_backend_c::{CFunctionStyle, ConfigBuilder, Generator};
use interoptopus_reference_project::ffi_inventory;
use tests::{compile_output_c, validate_output};

#[test]
fn forward() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().function_style(CFunctionStyle::ForwardDeclarations).build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests", "c_function_styles_forward.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}

#[test]
fn typedef() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().function_style(CFunctionStyle::Typedefs).build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests", " c_function_styles_typedefs.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
