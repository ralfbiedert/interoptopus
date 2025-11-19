//! Proc macros for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! Items in here will be re-exported by [the main crate](https://crates.io/crates/interoptopus).
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ffi(attr: TokenStream, item: TokenStream) -> TokenStream {
    interoptopus_proc_impl::ffi(attr.into(), item.into()).into()
}

#[proc_macro_derive(AsyncRuntime, attributes(runtime))]
pub fn derive_async_runtime(item: TokenStream) -> TokenStream {
    interoptopus_proc_impl::derive_async_runtime(item.into()).into()
}
