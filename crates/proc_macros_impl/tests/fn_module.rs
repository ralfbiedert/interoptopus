use quote::quote;
use syn::{parse_quote, ItemFn};

mod util;

#[test]
fn module_explicit() {
    let item: ItemFn = parse_quote! {
        #[ffi(module = "foo")]
        fn foo() {}
    };

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn module_common() {
    let item: ItemFn = parse_quote! {
        #[ffi(module = common)]
        fn foo() {}
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
