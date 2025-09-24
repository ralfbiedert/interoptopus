use quote::quote;
use syn::{parse_quote, ItemConst};

mod util;

#[test]
fn constant_literal() {
    let item: ItemConst = parse_quote! {
        #[ffi]
        const MY_CONST: u32 = 42;
    };

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn constant_function_call() {
    let item: ItemConst = parse_quote! {
        #[ffi]
        const COMPUTED: i32 = std::mem::size_of::<u64>() as i32;
    };

    insta::assert_snapshot!(expand_ffi!(item));
}
