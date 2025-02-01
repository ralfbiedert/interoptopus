use anyhow::Error;
use interoptopus::{ffi_function, function, Bindings, InventoryBuilder};
use interoptopus_backend_csharp::{DocConfig, Documentation, InteropBuilder};
use tests::backend_csharp::common_namespace_mappings;

/// Has documentation
#[ffi_function]
fn with_documentation() {}

#[test]
fn can_produce_markdown() -> Result<(), Error> {
    let inventory = InventoryBuilder::new().register(function!(with_documentation)).build();

    let interop = InteropBuilder::default()
        .inventory(inventory)
        .namespace_mappings(common_namespace_mappings())
        .build()?;

    let doc_config = DocConfig::default();
    let _doc_string = Documentation::new(&interop, doc_config).to_string()?;

    // validate_output!("tests", "csharp_markdown.md", doc_string.as_str());

    Ok(())
}
