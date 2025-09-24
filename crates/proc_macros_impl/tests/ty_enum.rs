use quote::quote;
use syn::{parse_quote, ItemEnum};

mod util;

#[test]
fn basic_enum() {
    let item: ItemEnum = parse_quote! {
        #[ffi]
        enum Foo {
            A,
            B
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
