use quote::quote;

#[test]
fn test_forbidden_struct_name() {
    let item_tokens = quote! {
        #[ffi_type]
        struct class {
            field: u32,
        }
    };

    let result = interoptopus_proc_impl::ffi_type(quote! {}, item_tokens);
    let output = result.to_string();
    assert!(output.contains("'class' is a forbidden name and cannot be used"));
}

#[test]
fn test_forbidden_enum_name() {
    let item_tokens = quote! {
        #[ffi_type]
        enum object {
            Variant1,
            Variant2(u32),
        }
    };

    let result = interoptopus_proc_impl::ffi_type(quote! {}, item_tokens);
    let output = result.to_string();
    assert!(output.contains("'object' is a forbidden name and cannot be used"));
}

#[test]
fn test_forbidden_field_name() {
    let item_tokens = quote! {
        #[ffi_type]
        struct MyStruct {
            class: u32,
            normal_field: i32,
        }
    };

    let result = interoptopus_proc_impl::ffi_type(quote! {}, item_tokens);
    let output = result.to_string();
    assert!(output.contains("'class' is a forbidden name and cannot be used as a field name"));
}

#[test]
fn test_forbidden_variant_name() {
    let item_tokens = quote! {
        #[ffi_type]
        enum MyEnum {
            class,
            ValidVariant(u32),
        }
    };

    let result = interoptopus_proc_impl::ffi_type(quote! {}, item_tokens);
    let output = result.to_string();
    assert!(output.contains("'class' is a forbidden name and cannot be used as a variant name"));
}

#[test]
fn test_forbidden_function_name() {
    let item_tokens = quote! {
        fn class() {}
    };

    let result = interoptopus_proc_impl::ffi_function(quote! {}, item_tokens);
    let output = result.to_string();
    assert!(output.contains("'class' is a forbidden name and cannot be used as a function name"));
}

#[test]
fn test_forbidden_parameter_name() {
    let item_tokens = quote! {
        fn my_function(class: u32, normal_param: i32) {}
    };

    let result = interoptopus_proc_impl::ffi_function(quote! {}, item_tokens);
    let output = result.to_string();
    assert!(output.contains("'class' is a forbidden name and cannot be used as a parameter name"));
}

#[test]
fn test_valid_names_are_allowed() {
    let struct_tokens = quote! {
        #[ffi_type]
        struct ValidStruct {
            valid_field: u32,
        }
    };

    let struct_result = interoptopus_proc_impl::ffi_type(quote! {}, struct_tokens);
    let struct_output = struct_result.to_string();
    assert!(!struct_output.contains("is a forbidden name"));

    let enum_tokens = quote! {
        #[ffi_type]
        enum ValidEnum {
            ValidVariant,
            AnotherValidVariant(u32),
        }
    };

    let enum_result = interoptopus_proc_impl::ffi_type(quote! {}, enum_tokens);
    let enum_output = enum_result.to_string();
    assert!(!enum_output.contains("is a forbidden name"));

    let function_tokens = quote! {
        fn valid_function(valid_param: u32) {}
    };

    let function_result = interoptopus_proc_impl::ffi_function(quote! {}, function_tokens);
    let function_output = function_result.to_string();
    assert!(!function_output.contains("is a forbidden name"));
}

#[test]
fn test_multiple_forbidden_names() {
    // Test that we catch the first forbidden name we encounter
    let item_tokens = quote! {
        #[ffi_type]
        struct ValidStruct {
            class: u32,
            object: i32,  // This should not be reached since class fails first
        }
    };

    let result = interoptopus_proc_impl::ffi_type(quote! {}, item_tokens);
    let output = result.to_string();
    assert!(output.contains("'class' is a forbidden name"));
}