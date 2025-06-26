use anyhow::Error;
use interoptopus::inventory::{Bindings, Inventory};
use interoptopus::{extra_type, ffi_type};
use interoptopus_backend_c::{EnumVariants, Interop};
use tests::{compile_output_c, validate_output};

#[ffi_type]
#[allow(unused)]
enum Color {
    Red,
    Green,
    Blue,
}

fn ffi_inventory() -> Inventory {
    Inventory::builder().register(extra_type!(Color)).build()
}

#[test]
fn with_enum_name() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .enum_variant_style(EnumVariants::WithEnumName)
        .build()?
        .to_string()?;

    validate_output!("tests", "c_variant_styles_with_name.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}

#[test]
fn variant_only() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .enum_variant_style(EnumVariants::VariantName)
        .build()?
        .to_string()?;

    validate_output!("tests", "c_variant_styles_variant_only.h", generated.as_str());
    compile_output_c!(generated.as_str());

    Ok(())
}
