use anyhow::Error;
use interoptopus_backend_csharp::{ConfigBuilder, DocConfig, DocGenerator, Generator, ParamSliceType};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn can_produce_markdown() -> Result<(), Error> {
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .param_slice_type(ParamSliceType::Span)
        .build()?;
    let generator = Generator::new(config, ffi_inventory());

    let doc_config = DocConfig::default();
    let doc_string = DocGenerator::new(&ffi_inventory(), &generator, doc_config).write_string()?;

    validate_output!("tests", "csharp_markdown.md", doc_string.as_str());

    Ok(())
}
