use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{FnArg, GenericParam, ItemFn, Pat, ReturnType, Signature, Type};

use crate::functions::Attributes;
use crate::util;

pub fn fn_signature_type(signature: &Signature) -> TokenStream {
    let rval = &signature.output;
    let abi = &signature.abi;
    let mut inputs = Vec::new();

    for input in &signature.inputs {
        match input {
            FnArg::Typed(x) => {
                inputs.push(x.ty.clone());
            }
            FnArg::Receiver(_) => panic!("Does not support receivers"),
        }
    }

    let span = signature.span();

    quote_spanned!(span=> #abi fn(#(#inputs),*) #rval)
}

pub fn rval_tokens(return_type: &ReturnType) -> TokenStream {
    let span = return_type.span();

    if let ReturnType::Type(_, x) = return_type {
        match &**x {
            Type::Path(x) => {
                let token = x.to_token_stream();
                quote_spanned!(span=> < #token as ::interoptopus::lang::rust::CTypeInfo>::type_info())
            }
            Type::Group(x) => {
                let token = x.to_token_stream();
                quote_spanned!(span=> < #token as ::interoptopus::lang::rust::CTypeInfo>::type_info())
            }
            Type::Tuple(_) => {
                // TODO: Check tuple is really empty.
                quote_spanned!(span=> ::interoptopus::lang::c::CType::Primitive(::interoptopus::lang::c::PrimitiveType::Void))
            }
            Type::Reference(x) => {
                let token = x.to_token_stream();
                quote_spanned!(span=> < #token as ::interoptopus::lang::rust::CTypeInfo>::type_info())
            }
            Type::Ptr(x) => {
                let token = x.to_token_stream();
                quote_spanned!(span=> < #token as ::interoptopus::lang::rust::CTypeInfo>::type_info())
            }
            Type::Array(x) => {
                let token = x.to_token_stream();
                quote_spanned!(span=> < #token as ::interoptopus::lang::rust::CTypeInfo>::type_info())
            }
            _ => {
                panic!("Unsupported type at interface boundary found for rval: {x:?}.")
            }
        }
    } else {
        quote_spanned!(span=> ::interoptopus::lang::c::CType::Primitive(interoptopus::lang::c::PrimitiveType::Void))
    }
}

#[allow(clippy::equatable_if_let, clippy::useless_let_if_seq)]
pub fn ffi_function_freestanding(_ffi_attributes: &Attributes, input: TokenStream) -> TokenStream {
    let mut item_fn = syn::parse2::<ItemFn>(input).expect("Must be a function.");
    let docs = util::extract_doc_lines(&item_fn.attrs);

    let mut args_name = Vec::new();
    let mut args_type = Vec::new();
    let mut generic_parameters = Vec::new();
    let mut generic_ident = Vec::new();

    let signature = fn_signature_type(&item_fn.sig);
    let rval = rval_tokens(&item_fn.sig.output);

    for generic in &item_fn.sig.generics.params {
        match generic {
            GenericParam::Type(_) => panic!("Generic types not supported in FFI functions."),
            GenericParam::Const(_) => panic!("Generic consts not supported in FFI functions."),
            GenericParam::Lifetime(lt) => {
                generic_parameters.push(lt.lifetime.to_token_stream());
                generic_ident.push(lt.lifetime.ident.clone());
            }
        }
    }

    let function_ident = item_fn.sig.ident.clone();
    let function_ident_str = function_ident.to_string();
    let mut generic_params = quote! {};
    let mut phantom_fields = quote! {};

    if !generic_parameters.is_empty() {
        generic_params = quote! { < #(#generic_parameters,)* > };
        phantom_fields = quote! {
            #(
                #generic_ident: ::std::marker::PhantomData<& #generic_parameters ()>,
            )*
        };
    }

    for arg in &item_fn.sig.inputs {
        if let FnArg::Typed(pat) = arg {
            let name = match pat.pat.as_ref() {
                Pat::Ident(ident) => ident.ident.to_string(),
                Pat::Wild(_) => "_ignored".to_string(),
                _ => {
                    panic!("Only supports normal identifiers for parameters, e.g., `x: ...`");
                }
            };

            let clean_name = name.strip_prefix('_').unwrap_or(&name);
            args_name.push(clean_name.to_string());

            let token = match util::purge_lifetimes_from_type(pat.ty.as_ref()) {
                Type::Path(x) => x.path.to_token_stream(),
                Type::Reference(x) => x.to_token_stream(),
                Type::Group(x) => x.to_token_stream(),
                Type::Ptr(x) => x.to_token_stream(),
                Type::Array(x) => x.to_token_stream(),
                Type::BareFn(x) => x.to_token_stream(),
                _ => {
                    panic!("Unsupported type at interface boundary found for parameter: {:?}.", pat.ty)
                }
            };

            args_type.push(quote! { < #token as ::interoptopus::lang::rust::CTypeInfo>::type_info() });
        } else {
            panic!("Does not support methods.")
        }
    }

    // Ensure we have the right attributes
    if item_fn.sig.abi.is_none() {
        item_fn.sig.abi = Some(syn::parse_quote!(extern "C"));
    }

    if !item_fn.attrs.iter().any(|attr| attr.path().is_ident("no_mangle")) {
        item_fn.attrs.push(syn::parse_quote!(#[no_mangle]));
    }

    let rval = quote! {
        #item_fn

        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate, clippy::forget_non_drop)]
        pub(crate) struct #function_ident #generic_params { #phantom_fields }

        unsafe impl #generic_params ::interoptopus::lang::rust::FunctionInfo for #function_ident #generic_params {
            type Signature = #signature;

            fn function_info() -> ::interoptopus::lang::c::Function {

                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();

                #(
                    params.push(::interoptopus::lang::c::Parameter::new(#args_name.to_string(), #args_type));
                )*

                #(
                    doc_lines.push(#docs.to_string());
                )*

                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(params, #rval);
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(doc_lines);
                let meta = ::interoptopus::lang::c::Meta::with_documentation(documentation);

                ::interoptopus::lang::c::Function::new(#function_ident_str.to_string(), signature, meta)
            }
        }
    };

    rval
}
