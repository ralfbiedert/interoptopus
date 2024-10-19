use anyhow::Error;
use interoptopus::Interop;
use interoptopus_backend_csharp::overloads::Unity;
use interoptopus_backend_csharp::{ConfigBuilder, Generator, Unsafe};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[test]
fn basic() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let overload = Unity::new();
    let config = ConfigBuilder::default()
        .namespace_mappings(common_namespace_mappings())
        .use_unsafe(Unsafe::UnsafeKeyword)
        .build()?;
    let generated = Generator::new(config, inventory).add_overload_writer(overload).write_string()?;

    validate_output!("tests", "csharp_overload_unity.cs", generated.as_str());

    Ok(())
}
