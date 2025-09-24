use quote::quote;
use syn::{parse_quote, ItemFn};

mod util;

#[test]
fn unique_name_is_stable() {
    let item: ItemFn = parse_quote! {
        #[ffi(export = unique)]
        fn foo() {}
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
