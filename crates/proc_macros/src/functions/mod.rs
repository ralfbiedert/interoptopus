use crate::macros::darling_parse;
use darling::FromMeta;
use freestanding::ffi_function_freestanding;
use proc_macro2::TokenStream;

mod freestanding;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    debug: bool,

    #[darling(default)]
    export_unique: bool,

    #[darling(default)]
    export_as: String,
}

pub fn ffi_function(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = darling_parse!(Attributes, attr);

    let rval = ffi_function_freestanding(&attributes, input);

    if attributes.debug {
        println!("{rval}");
    }

    rval
}
