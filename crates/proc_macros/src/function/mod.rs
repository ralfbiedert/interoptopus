use proc_macro2::TokenStream;

pub fn ffi_function(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    Ok(input.clone())
}
