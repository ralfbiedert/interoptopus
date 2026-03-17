use quote::quote;
use syn::{ItemImpl, parse_quote};

mod util;

#[test]
fn async_service() {
    let item: ItemImpl = parse_quote! {
        #[ffi]
        impl Service {
            pub fn create() -> ffi::Result<Self, Error> {
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

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn async_constructor() {
    let item: ItemImpl = parse_quote! {
        #[ffi]
        impl Service {
            pub async fn create(runtime: Async<Runtime>) -> ffi::Result<Self, Error> {
                ffi::Ok(Self)
            }

            pub fn method(&self) -> u32 {
                0
            }
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
