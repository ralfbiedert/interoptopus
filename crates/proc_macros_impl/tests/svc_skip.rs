use quote::quote;
use syn::{ItemImpl, parse_quote};

mod util;

#[test]
fn skip_impl() {
    let item: ItemImpl = parse_quote! {
        #[ffi]
        impl Service {
            #[ffi::skip]
            fn new() -> ffi::Result<Self, Error> { }
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
