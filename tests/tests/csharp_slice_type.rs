use anyhow::Error;
use interoptopus::Interop;
use interoptopus_backend_csharp::{ConfigBuilder, Generator, ParamSliceType, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn span() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .param_slice_type(ParamSliceType::Span)
        .build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests", "csharp_slice_type_span.cs", generated.as_str());

    Ok(())
}

#[test]
fn array() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .param_slice_type(ParamSliceType::Array)
        .build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests", "csharp_slice_type_array.cs", generated.as_str());

    Ok(())
}
