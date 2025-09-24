//! Proc macros for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! Items in here will be re-exported by [the main crate](https://crates.io/crates/interoptopus).
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro; // Apparently needed to be imported like this.

mod docs;
mod forbidden;
mod function;
mod service;
mod types;
mod utils;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Item};

pub fn ffi_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let item = proc_macro2::TokenStream::from(item);

    let rval = match types::ffi_type(attr, item.clone()) {
        Ok(result) => result,
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #item
                #error
            }
        }
    };

    rval.into()
}

pub fn ffi_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let item = proc_macro2::TokenStream::from(item);

    let rval = match function::ffi_function(attr, item.clone()) {
        Ok(result) => result,
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #item
                #error
            }
        }
    };

    rval.into()
}

pub fn ffi_service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let item = proc_macro2::TokenStream::from(item);

    let rval = match service::ffi_service(attr, item.clone()) {
        Ok(result) => result,
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #item
                #error
            }
        }
    };

    rval.into()
}

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
        },
    };

    // Parse and forward to appropriate macro based on item type
    let result = parse2::<Item>(item.clone()).and_then(|parsed_item| match parsed_item {
        Item::Struct(_) | Item::Enum(_) => types::ffi_type(attr, item.clone()),
        Item::Fn(_) => function::ffi_function(attr, item.clone()),
        Item::Impl(_) => service::ffi_service(attr, item.clone()),
        _ => Err(syn::Error::new_spanned(
            &parsed_item,
            "#[ffi] can only be applied to structs, enums, functions, or impl blocks",
        )),
    });

    handle_result(result).into()
}
