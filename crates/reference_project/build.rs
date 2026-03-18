fn main() {
    prost_build::compile_protos(&["proto/deeply_nested.proto"], &["proto/"]).unwrap();
}
