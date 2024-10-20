use anyhow::Error;
use interoptopus::{ffi_function, function, InventoryBuilder};
use interoptopus_backend_csharp::{ConfigBuilder, DocConfig, DocGenerator, Generator, ParamSliceType};
use tests::backend_csharp::common_namespace_mappings;

/// Has documentation
#[ffi_function]
fn with_documentation() {}

#[test]
fn can_produce_markdown() -> Result<(), Error> {
    let inventory = InventoryBuilder::new().register(function!(with_documentation)).inventory();

    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .param_slice_type(ParamSliceType::Span)
        .build()?;
    let generator = Generator::new(config, inventory.clone());

    let doc_config = DocConfig::default();
    let _doc_string = DocGenerator::new(&inventory, &generator, doc_config).write_string()?;

    // validate_output!("tests", "csharp_markdown.md", doc_string.as_str());

    Ok(())
}
