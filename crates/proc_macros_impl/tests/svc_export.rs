use quote::quote;
use syn::{ItemImpl, parse_quote};

mod util;

#[test]
fn export_unique() {
    let item: ItemImpl = parse_quote! {
        #[ffi(export = unique)]
        impl Service {
            fn create() -> ffi::Result<Self, Error> { }
            fn method(&self, x: u32) -> u32 { }
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn export_unique_is_stable() {
    let item1: ItemImpl = parse_quote! {
        #[ffi(export = unique)]
        impl Service {
            fn create() -> ffi::Result<Self, Error> { }
            fn method(&self, x: u32) -> u32 { }
        }
    };
    let item2: ItemImpl = parse_quote! {
        #[ffi(export = unique)]
        impl Service {
            fn create() -> ffi::Result<Self, Error> { }
            fn method(&self, x: u32) -> u32 { }
        }
    };

    assert_eq!(expand_ffi!(item1), expand_ffi!(item2));
}
