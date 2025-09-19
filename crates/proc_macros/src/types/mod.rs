use proc_macro2::TokenStream;

pub fn ffi_type(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
