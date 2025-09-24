//! Proc macros for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! Items in here will be re-exported by [the main crate](https://crates.io/crates/interoptopus).
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ffi_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    interoptopus_proc_impl::ffi_type(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn ffi_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    interoptopus_proc_impl::ffi_function(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn ffi_service(attr: TokenStream, item: TokenStream) -> TokenStream {
    interoptopus_proc_impl::ffi_service(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn ffi(attr: TokenStream, item: TokenStream) -> TokenStream {
    interoptopus_proc_impl::ffi(attr.into(), item.into()).into()
}
