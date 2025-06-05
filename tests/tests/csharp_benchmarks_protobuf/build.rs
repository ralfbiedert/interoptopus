//================
//=== PROTOBUF ===
//================

fn compile_proto_files() {
    let proto_files = ["Input.proto", "Outputs.proto"];
    let include_paths = ["/.", "./models"];

    for file in &proto_files {
        println!("cargo:rerun-if-changed={file}");
    }

    prost_build::compile_protos(&proto_files, &include_paths).unwrap();
}

//====================
//=== INTEROPTOPUS ===
//====================
use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus_backend_csharp::{InteropBuilder, WriteTypes};

use interoptopus::backend::NamespaceMappings;
pub fn namespace_mappings() -> NamespaceMappings {
    NamespaceMappings::new("Gen.ForCSharp")
        //.add("common", "Gen.ForCSharp.Common")
        .add("interopt_ffi", "Gen.ForCSharp.Ffi")
        .add("wire", "Gen.ForCSharp.Wire")
}

macro_rules! output {
    ($folder:expr, $file:expr, $generated:expr) => {
        let file = format!("{}/{}", $folder, $file);
        std::fs::write(file, $generated).unwrap();
    };
}

fn generate_interopt_files() -> Result<(), Error> {
    /*    let generated_common = InteropBuilder::new()
    .inventory(ffi_inventory())
    .namespace_mappings(namespace_mappings())
    .namespace_id("common".to_string())
    .dll_name("proto_benchy".to_string())
    .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
    .build()?
    .to_string()?; -- BROKEN */

    /*    let generated_wire = InteropBuilder::new()
    .inventory(ffi_inventory()) // FIXME: wire types are not part of inventory
    .namespace_mappings(namespace_mappings())
    .namespace_id("wire".to_string())
    .dll_name("proto_benchy".to_string())
    .write_types(WriteTypes::Namespace)
    .build()?
    .to_string()?;*/

    let generated_ffi = InteropBuilder::new()
        .inventory(interopt_ffi::ffi_inventory())
        .namespace_mappings(namespace_mappings())
        .namespace_id("interopt_ffi".to_string())
        .dll_name("proto_benchy".to_string())
        .write_types(WriteTypes::Namespace)
        .build()?
        .to_string()?;

    let generated = InteropBuilder::new()
        .inventory(interopt_ffi::ffi_inventory())
        .namespace_mappings(namespace_mappings())
        .dll_name("proto_benchy".to_string())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?
        .to_string()?;

    //    output!("./", "Interop.Common.cs", generated_common.as_str());
    //    output!("./", "Interop.Wire.cs", generated_wire.as_str());
    output!("./", "Interop.Ffi.cs", generated_ffi.as_str());
    output!("./", "Interop.cs", generated.as_str());

    Ok(())
}

fn main() {
    compile_proto_files();
    generate_interopt_files().unwrap();
}
