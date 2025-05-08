fn main() {
    let proto_files = ["Input.proto", "Outputs.proto"];
    let include_paths = ["./models"];

    for file in &proto_files {
        println!("cargo:rerun-if-changed={file}");
    }

    prost_build::compile_protos(&proto_files, &include_paths).unwrap();
}
