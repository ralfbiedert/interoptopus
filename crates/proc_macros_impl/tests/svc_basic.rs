use quote::quote;
use syn::{parse_quote, ItemImpl};

mod util;

#[test]
fn basic_service() {
    let item: ItemImpl = parse_quote! {
        #[ffi]
        impl Service {
            fn new() -> ffi::Result<Self, Error> { }
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
