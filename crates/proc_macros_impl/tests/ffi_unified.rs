use quote::quote;
use syn::{parse_quote, ItemFn, ItemStruct, ItemEnum, ItemImpl};

mod util;

#[test]
fn ffi_struct_expansion() {
    let item: ItemStruct = parse_quote! {
        #[ffi]
        pub struct TestStruct {
            pub x: u32,
            pub y: u32,
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn ffi_enum_expansion() {
    let item: ItemEnum = parse_quote! {
        #[ffi]
        pub enum TestEnum {
            Variant1,
            Variant2(u32),
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn ffi_function_expansion() {
    let item: ItemFn = parse_quote! {
        #[ffi]
        pub fn test_function(input: u32) -> u32 {
            input * 2
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}

#[test]
fn ffi_impl_expansion() {
    let item: ItemImpl = parse_quote! {
        #[ffi]
        impl TestService {
            pub fn new(value: u32) -> Self {
                Self { value }
            }

            pub fn get_value(&self) -> u32 {
                self.value
            }
        }
    };

    insta::assert_snapshot!(expand_ffi!(item));
}