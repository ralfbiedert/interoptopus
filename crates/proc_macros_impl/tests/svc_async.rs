use quote::quote;
use syn::{parse_quote, ItemImpl};

mod util;

#[test]
fn async_service() {
    let item: ItemImpl = parse_quote! {
        #[ffi_service]
        impl Service {
            pub fn new() -> ffi::Result<Self, Error> {
                ffi::Ok(Self)
            }

            pub async fn compute1(_: Async<Self>) -> ffi::Result<u8, Error> {
                ffi::Ok(0)
            }

            pub async fn compute2(_: Async<Self>) -> ffi::Result<u8, Error> {
                ffi::Ok(0)
            }
        }
    };

    insta::assert_snapshot!(expand_svc!(item));
}
