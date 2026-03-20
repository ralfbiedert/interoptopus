#![doc = include_str!("../README.md")]
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro;

use proc_macro::TokenStream;

/// See [`interoptopus::ffi`](https://docs.rs/interoptopus/latest/interoptopus/macro.ffi.html) for full documentation.
#[proc_macro_attribute]
pub fn ffi(attr: TokenStream, item: TokenStream) -> TokenStream {
    interoptopus_proc_impl::ffi(attr.into(), item.into()).into()
}

/// See [`interoptopus::AsyncRuntime`](https://docs.rs/interoptopus/latest/interoptopus/macro.AsyncRuntime.html) for full documentation.
#[proc_macro_derive(AsyncRuntime, attributes(runtime))]
pub fn derive_async_runtime(item: TokenStream) -> TokenStream {
    interoptopus_proc_impl::derive_async_runtime(item.into()).into()
}

// Declares a plugin interface for reverse interop (loading foreign plugins from Rust).
#[proc_macro]
pub fn plugin(input: TokenStream) -> TokenStream {
    interoptopus_proc_impl::plugin(input.into()).into()
}
