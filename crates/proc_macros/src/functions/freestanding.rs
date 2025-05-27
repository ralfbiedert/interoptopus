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
                // NB: type_info() is applicable only to Wire<T>
                let token = /*if x.path.segments[0].ident == "Wire" {
                    // Extract inner type from Wire<T>
                    if let syn::PathArguments::AngleBracketed(args) = &x.path.segments[0].arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                            inner_type.to_token_stream()
                        } else {
                            x.to_token_stream()
                        }
                    } else {
                        x.to_token_stream()
                    }
                } else {*/
                    x.to_token_stream()
                /* }*/;
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

/// Extracts the inner type T from Wire<T> if the given TypePath is Wire<T>, otherwise returns None
// fn _extract_inner_type_from_wire(type_path: &syn::TypePath) -> Option<Type> {
//     if type_path.path.segments[0].ident == "Wire" {
//         if let syn::PathArguments::AngleBracketed(args) = &type_path.path.segments[0].arguments {
//             if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
//                 return Some(inner_type.clone());
//             }
//         }
//     }
//     None
// }

/// Recursively wraps return statements in Wire<T>::from() calls
// fn wrap_return_statements(stmts: &mut Vec<syn::Stmt>, inner_type: &TokenStream) {
//     let stmts_len = stmts.len();
//     for (i, stmt) in stmts.iter_mut().enumerate() {
//         match stmt {
//             syn::Stmt::Expr(expr, semi) => {
//                 // If this is the last statement and has no semicolon, it's an implicit return
//                 if i == stmts_len - 1 && semi.is_none() {
//                     let wrapped = syn::parse_quote! {
//                         Wire::<#inner_type>::from(#expr)
//                     };
//                     *expr = wrapped;
//                 } else {
//                     wrap_return_in_expr(expr, inner_type);
//                 }
//             }
//             syn::Stmt::Local(local) => {
//                 if let Some(init) = &mut local.init {
//                     wrap_return_in_expr(&mut init.expr, inner_type);
//                 }
//             }
//             syn::Stmt::Item(_) => {}
//             syn::Stmt::Macro(_) => {}
//         }
//     }
// }

/// Recursively processes expressions to wrap return statements
// fn wrap_return_in_expr(expr: &mut syn::Expr, inner_type: &TokenStream) {
//     match expr {
//         syn::Expr::Return(ret_expr) => {
//             if let Some(return_value) = &mut ret_expr.expr {
//                 let wrapped = syn::parse_quote! {
//                     Wire::<#inner_type>::from(#return_value)
//                 };
//                 *return_value = Box::new(wrapped);
//             }
//         }
//         syn::Expr::Block(block_expr) => {
//             wrap_return_statements(&mut block_expr.block.stmts, inner_type);
//         }
//         syn::Expr::If(if_expr) => {
//             wrap_return_statements(&mut if_expr.then_branch.stmts, inner_type);
//             if let Some((_, else_branch)) = &mut if_expr.else_branch {
//                 wrap_return_in_expr(else_branch, inner_type);
//             }
//         }
//         syn::Expr::Match(match_expr) => {
//             for arm in &mut match_expr.arms {
//                 wrap_return_in_expr(&mut arm.body, inner_type);
//             }
//         }
//         syn::Expr::Loop(loop_expr) => {
//             wrap_return_statements(&mut loop_expr.body.stmts, inner_type);
//         }
//         syn::Expr::While(while_expr) => {
//             wrap_return_statements(&mut while_expr.body.stmts, inner_type);
//         }
//         syn::Expr::ForLoop(for_expr) => {
//             wrap_return_statements(&mut for_expr.body.stmts, inner_type);
//         }
//         syn::Expr::Closure(closure_expr) => {
//             wrap_return_in_expr(&mut closure_expr.body, inner_type);
//         }
//         _ => {}
//     }
// }

#[allow(clippy::equatable_if_let, clippy::useless_let_if_seq, clippy::too_many_lines)]
pub fn ffi_function_freestanding(ffi_attributes: &Attributes, input: TokenStream) -> TokenStream {
    let mut item_fn = syn::parse2::<ItemFn>(input).expect("Must be a function.");
    let docs = extract_doc_lines(&item_fn.attrs);

    let mut args_name = Vec::new();
    let mut args_type = Vec::new();
    let mut generic_parameters = Vec::new();
    let mut generic_ident = Vec::new();
    let mut wire_types = Vec::new();

    let signature = fn_signature_type(&item_fn.sig);
    let rval = rval_tokens(&item_fn.sig.output);

    // Handle return type - track Wire<T> types but don't modify signature
    if let ReturnType::Type(_arrow, return_type) = &item_fn.sig.output {
        match purge_lifetimes_from_type(return_type.as_ref()) {
            Type::Path(x) => {
                if x.path.segments[0].ident == "Wire" {
                    // Extract the inner type string for tracking
                    if let syn::PathArguments::AngleBracketed(args) = &x.path.segments[0].arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                            wire_types.push(quote! { < #inner_type as ::interoptopus::lang::WireInfo>::wire_info() });
                        }
                    }
                }
            }
            _ => {}
        }
    }

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
    let mut generic_params = quote! {};
    let mut phantom_fields = quote! {};

    let export_name = if !ffi_attributes.export_as.is_empty() {
        ffi_attributes.export_as.clone()
    } else if ffi_attributes.export_unique {
        let signature_tokens = quote::quote! { #item_fn.sig };
        let original_name = item_fn.sig.ident.to_string();
        let mut hasher = DefaultHasher::new();

        signature_tokens.to_string().hash(&mut hasher);
        let hash = hasher.finish();

        format!("{original_name}_{hash}")
    } else {
        function_ident.to_string()
    };

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

            // is this type Wire<T>? if so, add it to wired types list (probably via interoptopus::lang::Function::new?)

            let clean_name = name.strip_prefix('_').unwrap_or(&name);
            args_name.push(clean_name.to_string());

            let token = match purge_lifetimes_from_type(pat.ty.as_ref()) {
                Type::Path(x) => {
                    if x.path.segments[0].ident == "Wire" {
                        let wrapped_type = match &x.path.segments[0].arguments {
                            syn::PathArguments::AngleBracketed(a) => &a.args[0].to_token_stream(),
                            _ => unimplemented!(),
                        };
                        // Track this Wire<X> argument for type generation
                        wire_types.push(quote! { < #wrapped_type as ::interoptopus::lang::WireInfo>::wire_info() });
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

            // this makes a Wire<T> type_info here
            args_type.push(quote! { < #token as ::interoptopus::lang::TypeInfo>::type_info() });
        } else {
            panic!("Does not support methods.")
        }
    }

    // Ensure we have the right attributes
    if item_fn.sig.abi.is_none() {
        item_fn.sig.abi = Some(syn::parse_quote!(extern "C"));
    }

    if !item_fn.attrs.iter().any(|attr| attr.path().is_ident("no_mangle")) {
        item_fn.attrs.push(syn::parse_quote!(#[unsafe(no_mangle)]));
    }

    item_fn.attrs.push(syn::parse_quote!(#[unsafe(export_name = #export_name)]));

    // Generate preamble code for Wire<T> deserialization
    // if !wire_args.is_empty() {
    //     let mut preamble_stmts = Vec::new();

    //     for (arg_name, inner_type) in &wire_args {
    //         let arg_ident = syn::Ident::new(arg_name, proc_macro2::Span::call_site());
    //         let inner_type_tokens: TokenStream = inner_type.parse().expect("Failed to parse inner type");

    //         preamble_stmts.push(syn::parse_quote! {
    //             let #arg_ident: #inner_type_tokens = #arg_ident.deserialize().unwrap();
    //         });
    //     }

    //     // Prepend preamble statements to the function body
    //     let mut new_stmts = preamble_stmts;
    //     new_stmts.extend(item_fn.block.stmts.clone());
    //     item_fn.block.stmts = new_stmts;
    // }

    // Process return statements for Wire<T> wrapping
    // if let Some(inner_type_str) = &wire_return_type {
    //     let inner_type_tokens: TokenStream = inner_type_str.parse().expect("Failed to parse return inner type");
    //     wrap_return_statements(&mut item_fn.block.stmts, &inner_type_tokens);
    // }

    let rval = quote! {
        #item_fn

        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate, clippy::forget_non_drop)]
        pub struct #function_ident #generic_params { #phantom_fields }

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
                let meta = ::interoptopus::lang::Meta::with_docs(docs);

                let wire_types = vec![
                    #(#wire_types,)*
                ];

                ::interoptopus::lang::Function::new(#export_name.to_string(), sig, meta, wire_types)
            }

            // #wire_info
        }
    };

    rval
}
