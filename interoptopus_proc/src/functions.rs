use crate::functions::freestanding::ffi_function_freestanding;
use darling::FromMeta;
use proc_macro2::TokenStream;
use std::collections::HashMap;
use syn::AttributeArgs;

mod freestanding;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    surrogates: HashMap<String, String>,

    #[darling(default)]
    debug: bool,

    #[darling(default, rename = "unsafe")]
    unsfe: bool,
}

impl Attributes {
    pub fn assert_valid(&self) {
        if (!self.surrogates.is_empty()) && !self.unsfe {
            panic!("When using `surrogate` you must also specify `unsafe`.")
        }
    }
}

pub fn ffi_function(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let attributes: Attributes = Attributes::from_list(&attr).unwrap();
    attributes.assert_valid();

    let rval = ffi_function_freestanding(&attributes, input);

    if attributes.debug {
        println!("{}", rval.to_string());
    }

    rval
}
