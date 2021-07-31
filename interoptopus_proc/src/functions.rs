use crate::functions::freestanding::ffi_function_freestanding;
use crate::functions::service::ffi_function_service;
use crate::util;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{AttributeArgs, FnArg, GenericArgument, GenericParam, ItemFn, Pat, PathArguments, ReturnType, Signature, Type};

mod freestanding;
mod service;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    surrogates: HashMap<String, String>,

    #[darling(default)]
    debug: bool,

    #[darling(default, rename = "unsafe")]
    unsfe: bool,

    #[darling(default)]
    xxx_service: bool,
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

    let rval = if attributes.xxx_service {
        ffi_function_service(&attributes, input)
    } else {
        ffi_function_freestanding(&attributes, input)
    };

    if attributes.debug {
        println!("{}", rval.to_string());
    }

    rval
}
