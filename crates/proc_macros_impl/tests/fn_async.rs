use quote::quote;
use syn::{parse_quote, ItemFn};

mod util;

// TODO this should fail somehow and we want to check that
#[test]
fn async_fails() {
    let item: ItemFn = parse_quote! {
        #[ffi_function]
        async fn async_foo() {}
    };

    test_fn!(item);
}
