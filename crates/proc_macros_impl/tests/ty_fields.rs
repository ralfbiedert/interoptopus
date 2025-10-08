use quote::quote;
use syn::{ItemStruct, parse_quote};

mod util;

#[test]
fn fields() {
    let item: ItemStruct = parse_quote! {
        #[ffi]
        struct Foo {
            x: u8,
            y: String,
            z: f32,
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
