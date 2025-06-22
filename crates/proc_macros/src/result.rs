use crate::macros::darling_parse;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    asynk: bool,
}

#[allow(clippy::match_wildcard_for_single_variants)]
pub fn ffi_result(attr: TokenStream, input: &TokenStream) -> TokenStream {
    let attributes = darling_parse!(Attributes, attr);
    let item = syn::parse2::<ItemFn>(input.clone()).expect("Must be function.");

    let sig = item.sig;
    let body = &item.block;
    let stmts = &item.block.stmts;

    quote! {
        #sig {{
            use ::interoptopus::pattern::result::result_to_ffi;

            result_to_ffi(|| {{
                #(#stmts)*
            }})
        }}
    }
}
