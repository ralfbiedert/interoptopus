use interoptopus::proc::strip_module_paths;

#[test]
fn no_generics_no_path() {
    assert_eq!(strip_module_paths("Foo"), "Foo");
}

#[test]
fn no_generics_with_path() {
    assert_eq!(strip_module_paths("my_crate::module::Foo"), "Foo");
}

#[test]
fn simple_generic() {
    assert_eq!(strip_module_paths("crate::Enum<u32>"), "Enum<u32>");
}

#[test]
fn generic_with_qualified_param() {
    assert_eq!(strip_module_paths("crate::Struct<alloc::string::String>"), "Struct<String>");
}

#[test]
fn multiple_params() {
    assert_eq!(strip_module_paths("std::collections::HashMap<alloc::string::String, alloc::vec::Vec<u8>>"), "HashMap<String, Vec<u8>>");
}

#[test]
fn nested_generics() {
    assert_eq!(strip_module_paths("a::B<c::D<e::F>>"), "B<D<F>>");
}

#[test]
fn primitive_param() {
    assert_eq!(strip_module_paths("my::Wrapper<u32>"), "Wrapper<u32>");
}
