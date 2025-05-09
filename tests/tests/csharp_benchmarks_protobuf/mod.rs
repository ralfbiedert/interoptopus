// This is proto_benchy.dll doing two variants of the API:
// - one is Protobuf ser/de based
// - one is Wire<T> based
// The Wire<T> version does NOT need any protobuf files, and is defined
// solely using Rust types.
use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus_backend_csharp::{InteropBuilder, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

// mod models;
mod proto;
mod wire;

#[test]
fn prerequisites() -> Result<(), Error> {
    let generated_common = InteropBuilder::new()
        .inventory(ffi_inventory())
        .namespace_id("common".to_string())
        .namespace_mappings(common_namespace_mappings())
        .dll_name("proto_benchy".to_string())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?
        .to_string()?;

    let generated_other = InteropBuilder::new()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .dll_name("proto_benchy".to_string())
        .write_types(WriteTypes::Namespace)
        .build()?
        .to_string()?;

    validate_output!("tests/csharp_benchmarks_protobuf", "Interop.common.cs", generated_common.as_str());
    validate_output!("tests/csharp_benchmarks_protobuf", "Interop.cs", generated_other.as_str());

    Ok(())
}
