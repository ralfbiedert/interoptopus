mod args;
mod emit;
mod model;
mod validation;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemImpl};

use args::FfiServiceArgs;
use model::ServiceModel;

pub fn ffi_service(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let args: FfiServiceArgs = parse2(attr)?;
    let input_impl: ItemImpl = parse2(input)?;

    // Parse the model
    let model = ServiceModel::from_impl_item(input_impl.clone(), args.clone())?;

    // Validate the model
    model.validate(&input_impl)?;

    // Generate FFI snippets
    let ffi_functions = model.emit_ffi_functions()?;
    let service_info_impl = model.emit_service_info_impl()?;
    let validation_blocks = model.emit_const_verification_blocks()?;

    let result = quote! {
        #input_impl

        // Generated FFI functions
        #ffi_functions

        #service_info_impl
        #validation_blocks
    };

    if args.debug {
        match syn::parse2(result.clone()) {
            Ok(parsed) => {
                let formatted = prettyplease::unparse(&parsed);
                eprintln!("Generated code for service {}:\n{}", model.service_name, formatted);
            }
            Err(e) => {
                eprintln!("Failed to parse generated code for service {}: {}", model.service_name, e);
                eprintln!("Raw generated code:\n{}", result);
            }
        }
    }

    Ok(result)
}
