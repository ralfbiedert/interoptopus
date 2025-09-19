//! Proc macros for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! Items in here will be re-exported by [the main crate](https://crates.io/crates/interoptopus).

extern crate proc_macro; // Apparently needed to be imported like this.

mod types;

use proc_macro::TokenStream;

#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let input = proc_macro2::TokenStream::from(item);
    types::ffi_type(attr, input).into()
}
