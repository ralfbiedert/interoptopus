use quote::quote;
use syn::{parse_quote, ItemFn};

mod util;

#[test]
fn basic_expansion() {
    let item: ItemFn = parse_quote! {
        #[ffi_function]
        fn foo() {}
    };

    insta::assert_snapshot!(expand_fn!(item));
}
