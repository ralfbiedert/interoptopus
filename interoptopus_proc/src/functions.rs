use std::collections::HashMap;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, FnArg, ItemFn, Pat, ReturnType, Type};

use crate::util;
use syn::spanned::Spanned;

#[derive(Debug, FromMeta)]
pub struct FFIFunctionAttributes {
    #[darling(default)]
    surrogates: HashMap<String, String>,

    #[darling(default)]
    debug: bool,
}

pub fn ffi_function(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let item_fn: ItemFn = syn::parse2(input.clone()).expect("Must be item.");
    let ffi_attributes: FFIFunctionAttributes = FFIFunctionAttributes::from_list(&attr).unwrap();

    let span = item_fn.span();

    let function_ident = item_fn.sig.ident;
    let function_name = function_ident.to_string();

    let mut args_name = Vec::new();
    let mut args_type = Vec::new();

    let docs = util::extract_doc_lines(&item_fn.attrs);

    let rval = if let ReturnType::Type(_, x) = item_fn.sig.output {
        match *x {
            Type::Path(x) => {
                let token = x.to_token_stream();
                quote! { < #token as interoptopus::lang::rust::CTypeInfo>::type_info() }
            }
            Type::Group(x) => {
                let token = x.to_token_stream();
                quote! { < #token as interoptopus::lang::rust::CTypeInfo>::type_info() }
            }
            Type::Tuple(_) => {
                // TODO: Check tuple is really empty.
                quote! { interoptopus::lang::c::CType::Primitive(interoptopus::lang::c::PrimitiveType::Void) }
            }
            Type::Reference(x) => {
                let token = x.to_token_stream();
                quote! { < #token as interoptopus::lang::rust::CTypeInfo>::type_info() }
            }
            Type::Ptr(x) => {
                let token = x.to_token_stream();
                quote! { < #token as interoptopus::lang::rust::CTypeInfo>::type_info() }
            }
            _ => {
                panic!("Unsupported type at interface boundary found for rval: {:?}.", x)
            }
        }
    } else {
        quote! { interoptopus::lang::c::CType::Primitive(interoptopus::lang::c::PrimitiveType::Void) }
    };

    for arg in &item_fn.sig.inputs {
        if let FnArg::Typed(pat) = arg {
            let name = match pat.pat.as_ref() {
                Pat::Ident(ident) => ident.ident.to_string(),
                Pat::Wild(_) => "_".to_string(),
                _ => {
                    panic!("Only supports normal identifiers for parameters, e.g., `x: ...`");
                }
            };

            args_name.push(name.clone());

            if ffi_attributes.surrogates.contains_key(&name) {
                let lookup = ffi_attributes.surrogates.get(&name).unwrap();
                let ident = syn::Ident::new(&lookup, span);
                args_type.push(quote! { #ident()  })
            } else {
                let token = match pat.ty.as_ref() {
                    Type::Path(x) => x.path.to_token_stream(),
                    Type::Reference(x) => x.to_token_stream(),
                    Type::Group(x) => x.to_token_stream(),
                    Type::Ptr(x) => x.to_token_stream(),
                    _ => {
                        panic!("Unsupported type at interface boundary found for parameter: {:?}.", pat.ty)
                    }
                };

                args_type.push(quote! { < #token as interoptopus::lang::rust::CTypeInfo>::type_info() });
            }
        } else {
            panic!("Does not support methods.")
        }
    }

    let rval = quote! {
        #input

        #[allow(non_camel_case_types)]
        pub(crate) struct #function_ident {}

        impl interoptopus::lang::rust::FunctionInfo for #function_ident {
            fn function_info() -> interoptopus::lang::c::Function {

                let mut doc_lines = std::vec::Vec::new();
                let mut params = std::vec::Vec::new();

                #(
                    params.push(interoptopus::lang::c::Parameter::new(#args_name.to_string(), #args_type));
                )*

                #(
                    doc_lines.push(#docs.to_string());
                )*

                let mut signature = interoptopus::lang::c::FunctionSignature::new(params, #rval);
                let documentation = interoptopus::lang::c::Documentation::from_lines(doc_lines);
                let meta = interoptopus::lang::c::Meta::with_documentation(documentation);

                interoptopus::lang::c::Function::new(#function_name.to_string(), signature, meta)
            }
        }
    };

    if ffi_attributes.debug {
        println!("{}", rval.to_string());
    }

    rval
}
