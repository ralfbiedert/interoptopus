mod emit;
mod model;

use proc_macro2::TokenStream;
use syn::parse2;

use model::PluginModel;

pub fn plugin(input: TokenStream) -> TokenStream {
    match parse_and_emit(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn parse_and_emit(input: TokenStream) -> syn::Result<TokenStream> {
    let model = PluginModel::from_input(parse2(input)?)?;
    Ok(model.emit())
}
