use anyhow::Error;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_function, function, Interop, Inventory, InventoryBuilder};
use interoptopus_backend_csharp::overloads::DotNet;
use interoptopus_backend_csharp::{ConfigBuilder, Generator, ParamSliceType, Unsafe};
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[ffi_function]
fn sample_function(_: FFISlice<u8>) {}

fn ffi_inventory() -> Inventory {
    InventoryBuilder::new().register(function!(sample_function)).inventory()
}

#[test]
fn span() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .use_unsafe(Unsafe::UnsafePlatformMemCpy)
        .param_slice_type(ParamSliceType::Span)
        .build()?;
    let generated = Generator::new(config, inventory).add_overload_writer(DotNet::new()).write_string()?;

    validate_output!("tests", "csharp_slice_type_span.cs", generated.as_str());

    Ok(())
}

#[test]
fn array() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .use_unsafe(Unsafe::UnsafePlatformMemCpy)
        .param_slice_type(ParamSliceType::Array)
        .build()?;
    let generated = Generator::new(config, inventory).add_overload_writer(DotNet::new()).write_string()?;

    validate_output!("tests", "csharp_slice_type_array.cs", generated.as_str());

    Ok(())
}
