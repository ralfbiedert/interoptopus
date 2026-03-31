use quote::quote;
use syn::{parse_quote, ItemEnum};

mod util;

#[test]
fn variants_simple() {
    let item: ItemEnum = parse_quote! {
        #[ffi]
        enum Foo {
            A,
            B
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn variants_large() {
    let item: ItemEnum = parse_quote! {
        #[ffi]
        enum Foo {
            A,
            B = 16_000_000,
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn variants_negative() {
    let item: ItemEnum = parse_quote! {
        #[ffi]
        enum Foo {
            A,
            B = -2,
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
