//================
//=== PROTOBUF ===
//================

fn compile_proto_files() {
    let proto_files = ["Input.proto", "Outputs.proto"];
    let include_paths = ["../models"];

    if let Ok(entries) = std::fs::read_dir("../models") {
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
    println!("cargo:rerun-if-changed=../ffi/src/lib.rs");
    println!("cargo:rerun-if-changed=../ffi/src/ffi.rs");

    prost_build::compile_protos(&proto_files, &include_paths).unwrap();
}

fn main() {
    compile_proto_files();
}
