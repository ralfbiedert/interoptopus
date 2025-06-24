use proc_macro::TokenStream;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{parse_macro_input, File};

pub fn prettyprint_tokenstream(tokens: &TokenStream2) -> TokenStream {
    let rval1: proc_macro::TokenStream = tokens.clone().into();
    let syntax_tree: File = parse_macro_input!(rval1 as File);
    let string = prettyplease::unparse(&syntax_tree);
    println!("---------------------------------------------------------------------------------------------------");
    println!("{string}");
    syntax_tree.into_token_stream().into()
}
