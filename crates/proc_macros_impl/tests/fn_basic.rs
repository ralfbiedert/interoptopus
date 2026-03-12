use quote::quote;
use syn::{ItemFn, parse_quote};

mod util;

#[test]
fn basic_expansion() {
    let item: ItemFn = parse_quote! {
        #[ffi]
        fn foo() {}
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
