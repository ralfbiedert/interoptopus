use quote::quote;
use syn::{ItemStruct, parse_quote};

mod util;

#[test]
fn basic_enum() {
    let item: ItemStruct = parse_quote! {
        #[ffi_type]
        struct Foo {
            x: u8,
            #[ffi::skip]
            y: u8
        }
    };

    insta::assert_snapshot!(expand_ty!(item));
}
