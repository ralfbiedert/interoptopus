use crate::functions::freestanding::ffi_function_freestanding;
use darling::FromMeta;
use proc_macro2::TokenStream;
use syn::AttributeArgs;

mod freestanding;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    debug: bool,
}

pub fn ffi_function(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let attributes: Attributes = Attributes::from_list(&attr).unwrap();

    let rval = ffi_function_freestanding(&attributes, input);

    if attributes.debug {
        println!("{}", rval);
    }

    rval
}
