mod args;
mod emit;
mod model;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, parse2};

use args::FfiFunctionArgs;
use model::FunctionModel;

pub fn ffi_function(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let args: FfiFunctionArgs = parse2(attr)?;
    let input_fn: ItemFn = parse2(input)?;

    // Parse the model
    let model = FunctionModel::from_item_fn(input_fn.clone(), args.clone())?;

    // Generate the modified function
    let modified_function = model.emit_modified_function(&input_fn);

    // Generate the companion struct
    let companion_struct = model.emit_companion_struct();

    // Generate the FunctionInfo implementation
    let function_info_impl = model.emit_function_info_impl();

    let result = quote! {
        #modified_function
        #companion_struct
        #function_info_impl
    };

    if args.debug {
        let formatted = prettyplease::unparse(&syn::parse2(result.clone()).unwrap());
        eprintln!("Generated code for {}:\n{}", model.name, formatted);
    }

    Ok(result)
}
