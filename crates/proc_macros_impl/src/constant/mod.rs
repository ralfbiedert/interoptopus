use proc_macro2::TokenStream;

pub fn ffi(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    Ok(input)
}
