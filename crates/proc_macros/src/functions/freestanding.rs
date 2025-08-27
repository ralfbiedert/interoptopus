use crate::functions::Attributes;
use crate::util::{extract_doc_lines, purge_lifetimes_from_type};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use std::hash::{DefaultHasher, Hash, Hasher};
use syn::spanned::Spanned;
use syn::{FnArg, GenericParam, ItemFn, Pat, ReturnType, Signature, Type};

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
                quote_spanned!(span=> < #token as ::interoptopus::lang::TypeInfo>::type_info())
            }
            Type::Group(x) => {
                let token = x.to_token_stream();
                quote_spanned!(span=> < #token as ::interoptopus::lang::TypeInfo>::type_info())
            }
            Type::Tuple(_) => {
                // TODO: Check tuple is really empty.
                quote_spanned!(span=> ::interoptopus::lang::Type::Primitive(::interoptopus::lang::Primitive::Void))
            }
            Type::Reference(x) => {
                let token = x.to_token_stream();
                quote_spanned!(span=> < #token as ::interoptopus::lang::TypeInfo>::type_info())
            }
            Type::Ptr(x) => {
                let token = x.to_token_stream();
                quote_spanned!(span=> < #token as ::interoptopus::lang::TypeInfo>::type_info())
            }
            Type::Array(x) => {
                let token = x.to_token_stream();

                quote_spanned!(span=> < #token as ::interoptopus::lang::TypeInfo>::type_info())
            }
            _ => {
                panic!("Unsupported type at interface boundary found for rval: {x:?}.")
            }
        }
    } else {
        quote_spanned!(span=> ::interoptopus::lang::Type::Primitive(interoptopus::lang::Primitive::Void))
    }
}

/// Extract domain types from Wire<T> return type
fn process_return_type_domain_types(return_type: &ReturnType) -> Vec<TokenStream> {
    let mut domain_types = Vec::new();

    if let ReturnType::Type(_arrow, return_type) = return_type
        && let Type::Path(x) = purge_lifetimes_from_type(return_type.as_ref())
        && x.path.segments[0].ident == "Wire"
    {
        // Extract the inner type string for tracking
        if let syn::PathArguments::AngleBracketed(args) = &x.path.segments[0].arguments
            && let Some(syn::GenericArgument::Type(inner_type)) = args.args.first()
        {
            domain_types.push(quote! { < #inner_type as ::interoptopus::lang::WireInfo>::wire_info() });
        }
    }

    domain_types
}

/// Process generic parameters and return the necessary token streams
fn process_generic_parameters(generics: &syn::Generics) -> (Vec<TokenStream>, Vec<syn::Ident>, TokenStream, TokenStream) {
    let mut generic_parameters = Vec::new();
    let mut generic_ident = Vec::new();

    for generic in &generics.params {
        match generic {
            GenericParam::Type(_) => panic!("Generic types not supported in FFI functions."),
            GenericParam::Const(_) => panic!("Generic consts not supported in FFI functions."),
            GenericParam::Lifetime(lt) => {
                generic_parameters.push(lt.lifetime.to_token_stream());
                generic_ident.push(lt.lifetime.ident.clone());
            }
        }
    }

    let generic_params = if generic_parameters.is_empty() {
        quote! {}
    } else {
        quote! { < #(#generic_parameters,)* > }
    };

    let phantom_fields = if generic_parameters.is_empty() {
        quote! {}
    } else {
        quote! {
            #(
                #generic_ident: ::std::marker::PhantomData<& #generic_parameters ()>,
            )*
        }
    };

    (generic_parameters, generic_ident, generic_params, phantom_fields)
}

/// Determine the export name based on attributes
fn determine_export_name(ffi_attributes: &Attributes, function_ident: &syn::Ident, sig: &Signature) -> String {
    if !ffi_attributes.export_as.is_empty() {
        ffi_attributes.export_as.clone()
    } else if ffi_attributes.export_unique {
        let signature_tokens = quote::quote! { #sig };
        let original_name = function_ident.to_string();
        let mut hasher = DefaultHasher::new();

        signature_tokens.to_string().hash(&mut hasher);
        let hash = hasher.finish();

        format!("{original_name}_{hash}")
    } else {
        function_ident.to_string()
    }
}

/// Process a single function argument and return name, type token, and domain types
fn process_single_argument(pat: &syn::PatType) -> (String, TokenStream, Vec<TokenStream>) {
    let name = match pat.pat.as_ref() {
        Pat::Ident(ident) => ident.ident.to_string(),
        Pat::Wild(_) => "_ignored".to_string(),
        _ => {
            panic!("Only supports normal identifiers for parameters, e.g., `x: ...`");
        }
    };

    let clean_name = name.strip_prefix('_').unwrap_or(&name);
    let mut domain_types = Vec::new();

    let token = match purge_lifetimes_from_type(pat.ty.as_ref()) {
        Type::Path(x) => {
            if x.path.segments[0].ident == "Wire" {
                let wrapped_type = match &x.path.segments[0].arguments {
                    syn::PathArguments::AngleBracketed(a) => &a.args[0].to_token_stream(),
                    _ => unimplemented!(),
                };
                // For this Wire<X> argument track inner domain type X for type generation
                domain_types.push(quote! { < #wrapped_type as ::interoptopus::lang::WireInfo>::wire_info() });
            }
            x.path.to_token_stream()
        }
        Type::Reference(x) => x.to_token_stream(),
        Type::Group(x) => x.to_token_stream(),
        Type::Ptr(x) => x.to_token_stream(),
        Type::Array(x) => x.to_token_stream(),
        Type::BareFn(x) => x.to_token_stream(),
        _ => {
            panic!("Unsupported type at interface boundary found for parameter: {:?}.", pat.ty)
        }
    };

    let type_info = quote! { < #token as ::interoptopus::lang::TypeInfo>::type_info() };

    (clean_name.to_string(), type_info, domain_types)
}

/// Process function arguments and return parameter names, types, and domain types
fn process_function_arguments(inputs: &syn::punctuated::Punctuated<FnArg, syn::Token![,]>) -> (Vec<String>, Vec<TokenStream>, Vec<TokenStream>) {
    let mut args_name = Vec::new();
    let mut args_type = Vec::new();
    let mut domain_types = Vec::new();

    for arg in inputs {
        if let FnArg::Typed(pat) = arg {
            let (name, type_token, arg_domain_types) = process_single_argument(pat);
            args_name.push(name);
            args_type.push(type_token);
            domain_types.extend(arg_domain_types);
        } else {
            panic!("Does not support methods.")
        }
    }

    (args_name, args_type, domain_types)
}

/// Ensure the function has the proper FFI attributes
fn ensure_ffi_attributes(item_fn: &mut ItemFn, export_name: &str) {
    if item_fn.sig.abi.is_none() {
        item_fn.sig.abi = Some(syn::parse_quote!(extern "C"));
    }

    if !item_fn.attrs.iter().any(|attr| attr.path().is_ident("no_mangle")) {
        item_fn.attrs.push(syn::parse_quote!(#[unsafe(no_mangle)]));
    }

    item_fn.attrs.push(syn::parse_quote!(#[unsafe(export_name = #export_name)]));
}

pub fn ffi_function_freestanding(ffi_attributes: &Attributes, input: TokenStream) -> TokenStream {
    let namespace = ffi_attributes.namespace.clone().unwrap_or_default();
    let mut item_fn = syn::parse2::<ItemFn>(input).expect("Must be a function.");
    let docs = extract_doc_lines(&item_fn.attrs);

    let signature = fn_signature_type(&item_fn.sig);
    let rval = rval_tokens(&item_fn.sig.output);

    // Process return type for Wire<T> domain types
    let mut domain_types = process_return_type_domain_types(&item_fn.sig.output);

    // Process generic parameters
    let (_generic_parameters, _generic_ident, generic_params, phantom_fields) = process_generic_parameters(&item_fn.sig.generics);

    let function_ident = item_fn.sig.ident.clone();

    // Determine export name
    let export_name = determine_export_name(ffi_attributes, &function_ident, &item_fn.sig);

    // Process function arguments
    let (args_name, args_type, arg_domain_types) = process_function_arguments(&item_fn.sig.inputs);
    domain_types.extend(arg_domain_types);

    // Ensure proper FFI attributes
    ensure_ffi_attributes(&mut item_fn, &export_name);

    // Generate the final token stream
    quote! {
        #item_fn

        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate, clippy::forget_non_drop)]
        pub(crate) struct #function_ident #generic_params { #phantom_fields }

        unsafe impl #generic_params ::interoptopus::lang::FunctionInfo for #function_ident #generic_params {
            type Signature = #signature;

            fn function_info() -> ::interoptopus::lang::Function {

                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();

                #(
                    params.push(::interoptopus::lang::Parameter::new(#args_name.to_string(), #args_type));
                )*

                #(
                    doc_lines.push(#docs.to_string());
                )*

                let sig = ::interoptopus::lang::Signature::new(params, #rval);
                let docs = ::interoptopus::lang::Docs::from_lines(doc_lines);
                let meta = ::interoptopus::lang::Meta::with_module_docs(#namespace.to_string(), docs);

                let domain_types = vec![
                    #(#domain_types,)*
                ];

                ::interoptopus::lang::Function::new(#export_name.to_string(), sig, meta, domain_types)
            }

            // #wire_info
        }
    }
}
