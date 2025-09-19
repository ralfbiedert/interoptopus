use crate::macros::darling_parse;
use darling::FromMeta;
use freestanding::ffi_function_freestanding;
use proc_macro2::TokenStream;

mod freestanding;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    debug: bool,

    /// A function marked `export_unique` will be exported with a unique name. This is useful
    /// with generic functions inside macros, that need to be exported in multiple flavors,
    /// once per instantiated type.
    ///
    /// For example
    ///
    /// ```ignore
    /// #[ffi_function(export_unique)]
    /// fn vec_destroy(v: ffi::Vec<u32>) { ... }
    ///```
    ///
    /// might be exported as `vec_destroy_12831`.
    #[darling(default)]
    export_unique: bool,

    #[darling(default)]
    export_as: String,

    #[darling(default)]
    namespace: Option<String>,
}

pub fn ffi_function(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = darling_parse!(Attributes, attr);

    let rval = ffi_function_freestanding(&attributes, input);

    if attributes.debug {
        println!("{rval}");
    }

    rval
}
