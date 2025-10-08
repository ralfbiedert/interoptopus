mod args;
mod emit;
mod model;
mod validation;
mod wireio;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput};

use crate::skip::is_ffi_skip_attribute;

use args::FfiTypeArgs;
use model::TypeModel;

pub fn ffi(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let args: FfiTypeArgs = parse2(attr)?;
    let mut input_ast: DeriveInput = parse2(input)?;

    // Parse the model BEFORE removing skip attributes so it can detect them
    let model = TypeModel::from_derive_input(input_ast.clone(), args.clone())?;

    // Validate
    args.validate()?;
    model.validate()?;

    // Add repr attributes and remove skip attributes
    add_repr_attribute(&mut input_ast, &args)?;
    remove_skip_attributes(&mut input_ast);

    let typeinfo_impl = model.emit_typeinfo_impl()?;
    let wireio_impl = model.emit_wireio_impl();

    let result = quote! {
        #input_ast
        #typeinfo_impl
        #wireio_impl
    };

    if args.debug {
        let formatted = prettyplease::unparse(&syn::parse2(result.clone()).unwrap());
        eprintln!("Generated code for {}:\n{}", model.name, formatted);
    }

    Ok(result)
}

fn add_repr_attribute(input: &mut DeriveInput, args: &FfiTypeArgs) -> syn::Result<()> {
    if args.service {
        return Ok(());
    }

    // Check if a repr attribute already exists
    let has_repr = input.attrs.iter().any(|attr| attr.path().is_ident("repr"));

    if has_repr {
        return Ok(());
    }

    let repr_attr = if args.opaque {
        syn::parse_quote! { #[repr(C)] }
    } else if args.transparent {
        syn::parse_quote! { #[repr(transparent)] }
    } else if args.packed {
        syn::parse_quote! { #[repr(C, packed)] }
    } else {
        match &input.data {
            syn::Data::Struct(_) => syn::parse_quote! { #[repr(C)] },
            syn::Data::Enum(_) => syn::parse_quote! { #[repr(u32)] },
            syn::Data::Union(_) => return Err(syn::Error::new_spanned(input, "Unions are not supported")),
        }
    };

    input.attrs.push(repr_attr);
    Ok(())
}

fn remove_skip_attributes(input: &mut DeriveInput) {
    use syn::Data;

    match &mut input.data {
        Data::Struct(data_struct) => match &mut data_struct.fields {
            syn::Fields::Named(fields) => {
                for field in &mut fields.named {
                    field.attrs.retain(|attr| !is_ffi_skip_attribute(attr));
                }
            }
            syn::Fields::Unnamed(fields) => {
                for field in &mut fields.unnamed {
                    field.attrs.retain(|attr| !is_ffi_skip_attribute(attr));
                }
            }
            syn::Fields::Unit => {}
        },
        Data::Enum(data_enum) => {
            for variant in &mut data_enum.variants {
                variant.attrs.retain(|attr| !is_ffi_skip_attribute(attr));
                match &mut variant.fields {
                    syn::Fields::Named(fields) => {
                        for field in &mut fields.named {
                            field.attrs.retain(|attr| !is_ffi_skip_attribute(attr));
                        }
                    }
                    syn::Fields::Unnamed(fields) => {
                        for field in &mut fields.unnamed {
                            field.attrs.retain(|attr| !is_ffi_skip_attribute(attr));
                        }
                    }
                    syn::Fields::Unit => {}
                }
            }
        }
        Data::Union(_) => {}
    }
}
