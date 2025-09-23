use quote::quote;
use syn::{parse_quote, ItemStruct};

mod util;

#[test]
fn module_explicit() {
    let item: ItemStruct = parse_quote! {
        #[ffi_type(module = "foo")]
        struct Foo {
            x: u8
        }
    };

    insta::assert_snapshot!(expand_ty!(item));
}

#[test]
fn module_common() {
    let item: ItemStruct = parse_quote! {
        #[ffi_type(module = common)]
        struct Foo {
            x: u8
        }
    };

    insta::assert_snapshot!(expand_ty!(item));
}
