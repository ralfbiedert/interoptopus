use quote::quote;
use syn::{parse_quote, ItemFn};

mod util;

#[test]
fn module_explicit() {
    let item: ItemFn = parse_quote! {
        #[ffi_function(module = "foo")]
        fn foo() {}
    };

    insta::assert_snapshot!(expand_fn!(item));
}

#[test]
fn module_common() {
    let item: ItemFn = parse_quote! {
        #[ffi_function(module = "common")]
        fn foo() {}
    };

    insta::assert_snapshot!(expand_fn!(item));
}
