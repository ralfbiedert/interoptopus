mod args;
mod emit;
mod model;

use model::RuntimeModel;
use proc_macro2::TokenStream;
use syn::parse2;

pub fn derive_async_runtime(input: TokenStream) -> syn::Result<TokenStream> {
    let input_ast = parse2(input)?;
    let model = RuntimeModel::from_derive_input(input_ast)?;
    let impl_block = model.emit_async_runtime_impl();

    Ok(impl_block)
}