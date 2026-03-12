use quote::quote;
use syn::{ItemFn, parse_quote};

mod util;

// TODO this should fail somehow and we want to check that
#[test]
fn async_fails() {
    let item: ItemFn = parse_quote! {
        #[ffi]
        async fn async_foo() {}
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
