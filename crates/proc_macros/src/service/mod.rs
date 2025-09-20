use proc_macro2::TokenStream;

pub fn ffi_service(_attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    Ok(input)
}
