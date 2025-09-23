use quote::quote;

#[test]
fn test_no_mangle_rejected() {
    let item_tokens = quote! {
        #[no_mangle]
        fn foo() {}
    };

    let result = interoptopus_proc_impl::ffi_function(quote! {}, item_tokens);

    // The result should be an error containing the rejection message
    let output = result.to_string();
    assert!(output.contains("Functions with #[no_mangle] are not supported"));
}

#[test]
fn test_unsafe_no_mangle_rejected() {
    let item_tokens = quote! {
        #[unsafe(no_mangle)]
        fn foo() {}
    };

    let result = interoptopus_proc_impl::ffi_function(quote! {}, item_tokens);

    // The result should be an error containing the rejection message
    let output = result.to_string();
    assert!(output.contains("Functions with #[unsafe(no_mangle)] are not supported"));
}

#[test]
fn test_other_unsafe_attributes_allowed() {
    let item_tokens = quote! {
        #[unsafe(other_attribute)]
        fn foo() {}
    };

    let result = interoptopus_proc_impl::ffi_function(quote! {}, item_tokens);

    // This should succeed (not contain error about unsafe attributes)
    let output = result.to_string();
    assert!(!output.contains("Functions with #[unsafe"));
}