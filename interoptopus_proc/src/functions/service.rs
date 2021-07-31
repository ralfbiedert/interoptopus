use crate::functions::Attributes;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemFn, ItemImpl};

pub fn ffi_function_service(ffi_attributes: &Attributes, input: TokenStream) -> TokenStream {
    let item = syn::parse2::<ItemImpl>(input.clone()).expect("Must be item.");

    let rval = quote! {
        #input
    };

    input
}
