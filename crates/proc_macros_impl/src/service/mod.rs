mod args;
mod emit;
mod model;
mod validation;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemImpl};

use crate::skip::is_ffi_skip_attribute;

use args::FfiServiceArgs;
use model::ServiceModel;

pub fn ffi(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
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

    // Remove skip attributes from the impl block before outputting
    let mut cleaned_input_impl = input_impl;
    remove_skip_attributes(&mut cleaned_input_impl);

    let result = quote! {
        #validation_blocks

        #cleaned_input_impl

        // Generated FFI functions
        #ffi_functions

        #service_info_impl
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

/// Remove `ffi::skip` attributes from all methods in the impl block
fn remove_skip_attributes(input_impl: &mut ItemImpl) {
    for item in &mut input_impl.items {
        if let syn::ImplItem::Fn(method) = item {
            method.attrs.retain(|attr| !is_ffi_skip_attribute(attr));
        }
    }
}
