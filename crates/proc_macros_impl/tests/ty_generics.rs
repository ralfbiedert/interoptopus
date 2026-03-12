use quote::quote;
use syn::{ItemStruct, parse_quote};

mod util;

#[test]
fn generics() {
    let item: ItemStruct = parse_quote! {
        #[ffi]
        struct Foo<T: TypeInfo> {
            t: T,
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
