// TODO remove
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{FnArg, GenericParam, ItemFn, Pat, ReturnType, Signature, Type};

use crate::functions::Attributes;
use crate::surrogates::read_surrogates;
use crate::util;

pub fn fn_signature_type(signature: Signature) -> TokenStream {
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
                panic!("Unsupported type at interface boundary found for rval: {:?}.", x)
            }
        }
    } else {
        quote_spanned!(span=> ::interoptopus::lang::c::CType::Primitive(interoptopus::lang::c::PrimitiveType::Void))
    }
}

pub fn ffi_function_freestanding(ffi_attributes: &Attributes, input: TokenStream) -> TokenStream {
    let input: ItemFn = syn::parse2(input.clone()).expect("Must be item.");
    if is_augmented(&input) {
        ffi_function_freestanding_augmented(ffi_attributes, input)
    } else {
        ffi_function_freestanding_plain(ffi_attributes, input)
    }
}

// Ok sure this code isn't written elegantly at the moment. The idea is just that a function is
// considered Augmented if it returns a tuple type. We're just checking for that right now
//
// Theoretically you could decide to look for some other way to decide if a function is augmented.
// This is just a temporary example
fn is_augmented(function: &ItemFn) -> bool {
    match &function.sig.output {
        syn::ReturnType::Type(_, ty) => match &**ty {
            // need to check for length > 1, because returning `()` is considered a tuple
            //
            // You could also decide that that should become an augmented type instead and thus not
            // perform the check
            syn::Type::Tuple(syn::TypeTuple{elems, ..}) => elems.len() > 1,
            _other => false,
        },
        _other => false
    }
}

pub fn ffi_function_freestanding_plain(_ffi_attributes: &Attributes, item_fn: ItemFn) -> TokenStream {
    let docs = util::extract_doc_lines(&item_fn.attrs);

    let mut args_name = Vec::new();
    let mut args_type = Vec::new();
    let mut generic_parameters = Vec::new();
    let mut generic_ident = Vec::new();

    let signature = fn_signature_type(item_fn.sig.clone());
    let rval = rval_tokens(&item_fn.sig.output);
    let surrogates = read_surrogates(&item_fn.attrs);

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

    let _ = item_fn
        .sig
        .abi
        .clone()
        .unwrap_or_else(|| panic!(r#"Function '{}' must have ABI specifier such as 'extern "C"'."#, function_ident_str));

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

            if surrogates.1.contains_key(&name) {
                let lookup = surrogates.1.get(&name).unwrap();
                let ident = syn::Ident::new(lookup, surrogates.0.unwrap());
                args_type.push(quote! { #ident()  })
            } else {
                args_type.push(quote! { < #token as ::interoptopus::lang::rust::CTypeInfo>::type_info() });
            }
        } else {
            panic!("Does not support methods.")
        }
    }

   let input = item_fn.to_token_stream();
    let rval = quote! {
        #input

        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
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
                let meta = ::interoptopus::lang::c::Meta::with_documentation(documentation, None);

                ::interoptopus::lang::c::Function::new(#function_ident_str.to_string(), signature, meta)
            }
        }
    };

    rval
}

// This function will modify the original function's name such that it becomes a private internal
// function. We will call this function for now the "base_function". It will also write out a
// secondary function that has the original function's name, but that converts the augmented
// function into something that is C-ABI compatible. This function will be called
// "augmented_function" for now
pub fn ffi_function_freestanding_augmented(_ffi_attributes: &Attributes, item_fn: ItemFn) -> TokenStream {
    // ffi_function_freestanding_plain(_ffi_attributes, item_fn)
    // todo!("augmented function not implemented")

    let mut base_function = item_fn.clone();
    let base_function_name = format!("{}_interoptopus_internal", item_fn.sig.ident);
    base_function.sig.ident = syn::Ident::new(&base_function_name, item_fn.sig.ident.span());

    let augmented_function_name = item_fn.sig.ident.clone();

    let augmented_function_params = item_fn.sig.inputs.clone();
    let base_return_values = match &base_function.sig.output {
        // TODO this match should be done earlier and the parameters passed directly into this
        // function as arguments
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Tuple(t) => t.clone(),
            _other => panic!("should not have gotten here"),
        }
        _other => panic!("should not have gotten here"),
    };

    let mut augmented_params = base_function.sig.inputs.clone();

    for (i, return_value) in base_return_values.elems.iter().enumerate() {
        augmented_params.push(
            syn::FnArg::Typed(syn::PatType{
                attrs: Vec::new(),
                pat: Box::new(
                    syn::Pat::Ident( syn::PatIdent{
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: syn::Ident::new(&format!("out_param_{i}"), item_fn.span()),
                        subpat: None,
                    })
                ),
                colon_token: syn::token::Colon {
                    spans: [item_fn.span()],
                },
                ty: Box::new(
                    syn::Type::Ptr (syn::TypePtr{
                        star_token: syn::token::Star {
                            spans: [item_fn.span()],
                        },
                        const_token: None,
                        mutability: Some(syn::token::Mut {
                            span: item_fn.span(),
                        }),
                        elem: Box::new(return_value.clone()),
                    })
                )

            })
        );
    }

    // let augmented_args = quote!(#(#base_return_values.elems),*);

    let base_function_params = base_function.sig.inputs.iter().map(
        |fn_arg| {
            match fn_arg {
                syn::FnArg::Typed(p) => match &*p.pat {
                    syn::Pat::Ident(i) => i.ident.clone(),
                    _other => panic!("why can't I match on boxes in Rust?"),
                }
                _other => panic!("should not have gotten here"),
            }
        }
    ).collect::<Vec<_>>();

    let mut augmented_function_signature = item_fn.sig.clone();
    augmented_function_signature.output = syn::ReturnType::Default;
    augmented_function_signature.inputs = augmented_params.clone();

    let base_function_name = base_function.sig.ident.clone();
    let base_function = base_function.to_token_stream();
    let augmented_params = augmented_params.to_token_stream();

    let return_values = base_return_values.elems.iter().enumerate().map(|(i, _)| syn::Ident::new(&format!("return_value_{i}"), item_fn.span())).collect::<Vec<_>>();

    let write_to_out_params_snippets = return_values.iter().enumerate().map(|(i, return_value)|{
        let dest = syn::Ident::new(&format!("out_param_{i}"), item_fn.span());
        quote!{
            *#dest = #return_value
        }
    }).collect::<Vec<_>>();


    // TODO this is basically just copy-pasted, I'm not yet sure what this is doing
    let function_ident = item_fn.sig.ident.clone();
    let function_ident_str = function_ident.to_string();
    let mut generic_params = quote! {};
    let mut phantom_fields = quote! {};
    let signature = fn_signature_type(augmented_function_signature);
    let mut args_name = Vec::<String>::new();
    let mut args_type = Vec::<TokenStream>::new();
    let docs = Vec::<String>::new();

    let rval = quote! {
        // write out the original base function
        #base_function

        // write out the augmented function
        /// TODO put the documentation strings here
        #[no_mangle]
        pub extern "C" fn #augmented_function_name(#augmented_params) {
            let (#(#return_values),*) = #base_function_name(#(#base_function_params),*);

            unsafe {
                #(#write_to_out_params_snippets);*
            }
        }

        // TODO it's probably necessary to pass the function info here
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
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

                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(params, ::interoptopus::lang::c::CType::Primitive(::interoptopus::lang::c::PrimitiveType::Void));
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(doc_lines);
                let meta = ::interoptopus::lang::c::Meta::with_documentation(documentation, None);

                ::interoptopus::lang::c::Function::new(#function_ident_str.to_string(), signature, meta)
            }
        }
    };

    rval
}
