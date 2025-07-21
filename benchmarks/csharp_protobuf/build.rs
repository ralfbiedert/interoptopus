//================
//=== PROTOBUF ===
//================

fn compile_proto_files() {
    let proto_files = ["Input.proto", "Outputs.proto"];
    let include_paths = ["/.", "./models"];

    if let Ok(entries) = std::fs::read_dir("models") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("proto") {
                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
    }

    // Force rerun when interop source files change
    println!("cargo:rerun-if-changed=interop/src/lib.rs");
    println!("cargo:rerun-if-changed=interop/src/ffi.rs");

    prost_build::compile_protos(&proto_files, &include_paths).unwrap();
}

//====================
//=== INTEROPTOPUS ===
//====================
use anyhow::Error;
use interoptopus::inventory::Bindings;
use interoptopus_backend_csharp::{Interop, WriteTypes};

use interoptopus::backend::NamespaceMappings;
pub fn namespace_mappings() -> NamespaceMappings {
    NamespaceMappings::new("Gen.ForCSharp") //.add("ffi", "Gen.FFI").add("wire", "Gen.Wire")
}

fn generate_interop_files() -> Result<(), Error> {
    Interop::builder()
        .inventory(interop::ffi_inventory())
        .namespace_mappings(namespace_mappings())
        .dll_name("proto_benchy".to_string())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?
        .write_file("./Interop.cs")?;

    // Force rerun when interop source files change
    println!("cargo:rerun-if-changed=interop/src/wire.rs");

    Ok(())
}

fn main() {
    compile_proto_files();
    generate_interop_files().unwrap();
}
