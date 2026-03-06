use interoptopus_backends::casing::{rust_to_pascal, sanitize_delegate_name, sanitize_rust_name};

#[test]
fn pascal_simple() {
    assert_eq!(rust_to_pascal("my_type"), "MyType");
}

#[test]
fn pascal_already_capitalized() {
    assert_eq!(rust_to_pascal("MyStruct"), "MyStruct");
}

#[test]
fn pascal_spaces() {
    assert_eq!(rust_to_pascal("vec3 f32"), "Vec3F32");
}

#[test]
fn sanitize_name_generic() {
    assert_eq!(sanitize_rust_name("Weird2<u8, 5>"), "Weird2U85");
}

#[test]
fn sanitize_name_array() {
    assert_eq!(sanitize_rust_name("[u8; 5]"), "U85");
}

#[test]
fn sanitize_name_plain() {
    assert_eq!(sanitize_rust_name("MyStruct"), "MyStruct");
}

#[test]
fn sanitize_name_underscores() {
    assert_eq!(sanitize_rust_name("my_struct"), "MyStruct");
}

#[test]
fn sanitize_delegate_extern_c() {
    assert_eq!(sanitize_delegate_name("extern \"C\" fn(u8) -> u8"), "FnU8U8");
}

#[test]
fn sanitize_delegate_void_return() {
    assert_eq!(sanitize_delegate_name("extern \"C\" fn(Vec3f32) -> ()"), "FnVec3f32");
}

#[test]
fn sanitize_delegate_no_extern_prefix() {
    assert_eq!(sanitize_delegate_name("fn(u32, u32) -> bool"), "FnU32U32Bool");
}

#[test]
fn sanitize_delegate_no_args() {
    assert_eq!(sanitize_delegate_name("extern \"C\" fn() -> u32"), "FnU32");
}
