use quote::quote;
use syn::{ItemStruct, parse_quote};

mod util;

#[test]
fn skip_field() {
    let item: ItemStruct = parse_quote! {
        #[ffi_type]
        struct Foo {
            x: u8,
            #[ffi::skip]
            y: PhantomData<()>
        }
    };

    insta::assert_snapshot!(expand_ty!(item));
}
