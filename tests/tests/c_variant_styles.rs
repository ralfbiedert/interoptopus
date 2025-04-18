use anyhow::Error;
use interoptopus::inventory::{Bindings, Inventory, InventoryBuilder};
use interoptopus::{ffi_function, function};
use interoptopus_backend_c::{EnumVariants, InteropBuilder};
use tests::{compile_output_c, validate_output};

#[ffi_function]
fn sample_function() {}

fn ffi_inventory() -> Inventory {
    InventoryBuilder::new().register(function!(sample_function)).build()
}

#[test]
fn with_enum_name() -> Result<(), Error> {
    let generated = InteropBuilder::new()
        .inventory(ffi_inventory())
        .enum_variant_style(EnumVariants::WithEnumName)
        .build()?
        .to_string()?;

    validate_output!("tests", "c_variant_style_with_name.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}

#[test]
fn variant_only() -> Result<(), Error> {
    let generated = InteropBuilder::new()
        .inventory(interoptopus_reference_project::ffi_inventory())
        .enum_variant_style(EnumVariants::VariantName)
        .build()?
        .to_string()?;

    validate_output!("tests", "c_variant_style_variant_only.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
