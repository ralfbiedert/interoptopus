use quote::quote;
use syn::{parse_quote, ItemConst, ItemFn};

mod util;

#[test]
fn basic_expansion() {
    let item: ItemConst = parse_quote! {
        #[ffi]
        const X: u8 = 123;
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
