use quote::quote;
use syn::{ItemImpl, parse_quote};

mod util;

#[test]
fn basic_service() {
    let item: ItemImpl = parse_quote! {
        #[ffi_services]
        impl Service {
            fn new() -> ffi::Result<Self, Error> { }
        }
    };

    insta::assert_snapshot!(expand_svc!(item));
}
