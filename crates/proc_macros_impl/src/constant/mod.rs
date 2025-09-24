mod args;
mod emit;
mod model;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemConst};

use args::FfiConstantArgs;
use model::ConstantModel;

pub fn ffi(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let args: FfiConstantArgs = parse2(attr)?;
    let input_const: ItemConst = parse2(input)?;

    // Create the model from the parsed input
    let model = ConstantModel::from_item_const(input_const.clone(), args.clone())?;

    // Validate the model
    args.validate()?;
    model.validate()?;

    // Generate the ConstantInfo implementation
    let constant_info_impl = model.emit_constant_info_impl()?;

    let result = quote! {
        #input_const
        #constant_info_impl
    };

    if args.debug {
        let formatted = prettyplease::unparse(&syn::parse2(result.clone()).unwrap());
        eprintln!("Generated code for constant {}:\n{}", model.name, formatted);
    }

    Ok(result)
}
