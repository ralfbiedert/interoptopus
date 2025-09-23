use quote::quote;
use syn::{parse_quote, ItemFn, ItemStruct};

mod util;

#[test]
fn test_type_emission_external_default() {
    // Test default behavior - no module specified
    let item: ItemStruct = parse_quote! {
        #[ffi_type]
        struct MyStruct {
            data: u32,
        }
    };

    let output = expand_ty!(item);
    assert!(output.contains("Emission::External"), "Expected External emission for types with no module");
}

#[test]
fn test_type_emission_module_string() {
    // Test module with string value
    let item: ItemStruct = parse_quote! {
        #[ffi_type(module = "my_module")]
        struct MyStruct {
            data: u32,
        }
    };

    let output = expand_ty!(item);
    assert!(output.contains("Emission::Module"), "Expected Module emission for types with module string");
    assert!(output.contains("my_module"), "Expected module name in emission");
}

#[test]
fn test_type_emission_module_common() {
    // Test module with 'common' identifier
    let item: ItemStruct = parse_quote! {
        #[ffi_type(module = common)]
        struct MyStruct {
            data: u32,
        }
    };

    let output = expand_ty!(item);
    assert!(output.contains("Emission::Common"), "Expected Common emission for types with module = common");
}

#[test]
fn test_function_emission_external_default() {
    // Test default behavior - no module specified
    let item: ItemFn = parse_quote! {
        #[ffi_function]
        fn my_function() {}
    };

    let output = expand_fn!(item);
    assert!(output.contains("Emission::External"), "Expected External emission for functions with no module");
}

#[test]
fn test_function_emission_module_string() {
    // Test module with string value
    let item: ItemFn = parse_quote! {
        #[ffi_function(module = "my_module")]
        fn my_function() {}
    };

    let output = expand_fn!(item);
    assert!(output.contains("Emission::Module"), "Expected Module emission for functions with module string");
    assert!(output.contains("my_module"), "Expected module name in emission");
}

#[test]
fn test_function_emission_module_common() {
    // Test module with 'common' identifier
    let item: ItemFn = parse_quote! {
        #[ffi_function(module = common)]
        fn my_function() {}
    };

    let output = expand_fn!(item);
    assert!(output.contains("Emission::Common"), "Expected Common emission for functions with module = common");
}