use crate::functions::freestanding::ffi_function_freestanding;
use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro2::TokenStream;

mod freestanding;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    debug: bool,
}

pub fn ffi_function(attr: TokenStream, input: TokenStream) -> TokenStream {
    let nested_meta = NestedMeta::parse_meta_list(attr).unwrap();
    let attributes: Attributes = Attributes::from_list(&nested_meta).unwrap();

    let rval = ffi_function_freestanding(&attributes, input);

    if attributes.debug {
        println!("{}", rval);
    }

    rval
}
