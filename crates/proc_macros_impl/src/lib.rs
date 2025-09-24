//! Proc macros for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! Items in here will be re-exported by [the main crate](https://crates.io/crates/interoptopus).
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro; // Apparently needed to be imported like this.

mod constant;
mod docs;
mod forbidden;
mod function;
mod service;
mod skip;
mod types;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Item};

pub fn ffi(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let item = proc_macro2::TokenStream::from(item);

    // Helper function to handle results with consistent error formatting
    let handle_result = |result: syn::Result<TokenStream>| match result {
        Ok(tokens) => tokens,
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #item
                #error
            }
        }
    };

    // Parse and forward to appropriate macro based on item type
    let result = parse2::<Item>(item.clone()).and_then(|parsed_item| match parsed_item {
        Item::Struct(_) | Item::Enum(_) => types::ffi(attr, item.clone()),
        Item::Fn(_) => function::ffi(attr, item.clone()),
        Item::Const(_) => constant::ffi(attr, item.clone()),
        Item::Impl(_) => service::ffi(attr, item.clone()),
        _ => Err(syn::Error::new_spanned(&parsed_item, "#[ffi] can only be applied to structs, enums, functions, const, or impl blocks")),
    });

    handle_result(result).into()
}
