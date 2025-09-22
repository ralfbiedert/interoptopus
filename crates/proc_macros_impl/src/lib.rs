//! Proc macros for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! Items in here will be re-exported by [the main crate](https://crates.io/crates/interoptopus).
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro; // Apparently needed to be imported like this.

mod common;
mod function;
mod service;
mod types;

use proc_macro2::TokenStream;
use quote::quote;

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
