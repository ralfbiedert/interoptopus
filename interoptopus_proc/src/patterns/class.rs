use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{AttributeArgs, FnArg, ImplItem, ImplItemMethod, ItemImpl, Pat, PatType, Type};

use crate::util::extract_doc_lines;
use std::ops::Deref;
use syn::spanned::Spanned;

#[derive(Debug, FromMeta)]
pub struct FFIClassAttributes {
    #[darling(default)]
    error: String,
}

pub fn constructor(attr: &FFIClassAttributes, type_name: &str, span: Span) -> TokenStream {
    let ctor = syn::Ident::new(&format!("{}_create", type_name.to_lowercase()), span);
    let type_ident = syn::Ident::new(&type_name, span);

    quote! {
        #[no_mangle]
        pub extern "C" fn #ctor (context_ptr: Option<&mut *mut #type_ident>) -> < #type_ident as interoptopus::patterns::class::ClassPattern >::FFIError {
            interoptopus::patterns::success_enum::error_to_ffi_error(|| {
                let context_ref = context_ptr.ok_or(< #type_ident as interoptopus::patterns::class::ClassPattern >::null_error())?;
                let boxed = Box::new(#type_ident::new()?);
                let raw = Box::into_raw(boxed);

                *context_ref = raw;

                Ok::<(), < #type_ident as interoptopus::patterns::class::ClassPattern >::Error>(())
            })
        }
    }
}

pub fn deconstructor(attr: &FFIClassAttributes, type_name: &str, span: Span) -> TokenStream {
    let dtor = syn::Ident::new(&format!("{}_destroy", type_name.to_lowercase()), span.clone());
    let type_ident = syn::Ident::new(&type_name, span.clone());

    quote! {
        #[no_mangle]
        pub extern "C" fn #dtor (context_ptr: Option<&mut *mut #type_ident>) -> < #type_ident as interoptopus::patterns::class::ClassPattern >::FFIError {
            interoptopus::patterns::success_enum::error_to_ffi_error(|| {
                let context = context_ptr.ok_or(< #type_ident as interoptopus::patterns::class::ClassPattern >::null_error())?;

                {
                    unsafe { Box::from_raw(*context) };
                }

                *context = std::ptr::null_mut();

                Ok::<(), < #type_ident as interoptopus::patterns::class::ClassPattern >::Error>(())
            })
        }
    }
}

pub fn regular_function(main: &ItemImpl, method: &ImplItemMethod, type_name: String, method_name: String) -> TokenStream {
    let fn_name = syn::Ident::new(&format!("{}_{}", type_name.to_lowercase(), method_name), main.span());
    let method_ident = syn::Ident::new(&method_name, main.span());
    let type_ident = syn::Ident::new(&type_name, main.span());

    let mut args_with_type = vec![quote! { context_ptr: Option<&mut #type_ident>}];
    let mut args_name = vec![];

    for other_arg in method.sig.inputs.iter().skip(1) {
        args_with_type.push(other_arg.to_token_stream());

        match other_arg {
            FnArg::Receiver(_) => panic!("Argument must be regular named argument like `x: T`."),
            FnArg::Typed(x) => match x.pat.deref() {
                Pat::Ident(x) => args_name.push(x.ident.to_token_stream()),
                _ => {
                    panic!("Argument must be regular named argument like `x: T`.")
                }
            },
        }
    }

    quote! {
        #[no_mangle]
        pub extern "C" fn #fn_name(#(#args_with_type),*) -> < #type_ident as interoptopus::patterns::class::ClassPattern >::FFIError {
             interoptopus::patterns::success_enum::error_to_ffi_error(|| {
                let context = context_ptr.ok_or(< #type_ident as interoptopus::patterns::class::ClassPattern >::null_error())?;

                context.#method_ident(#(#args_name),*)?;

                Ok::<(), < #type_ident as interoptopus::patterns::class::ClassPattern >::Error>(())
            })

        }
    }
}

pub fn create_c_function(attr: &FFIClassAttributes, main: &ItemImpl, item: &ImplItem) -> Option<TokenStream> {
    let self_ty = main.self_ty.deref();
    let method = match item {
        ImplItem::Method(x) => x,
        _ => return None,
    };
    let method_name = method.sig.ident.to_string();
    let type_name = match self_ty {
        Type::Path(path) => path.path.to_token_stream().to_string(),
        _ => panic!("Unable to determine good name for class"),
    };

    let raw_function = regular_function(main, method, type_name, method_name);

    // We also send this through our ffi_function macro to we get the right info
    Some(crate::ffi_function(proc_macro::TokenStream::new(), raw_function.into()).into())
}

pub fn ffi_class(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let attr: FFIClassAttributes = FFIClassAttributes::from_list(&attr).unwrap();
    let impl_item: ItemImpl = syn::parse2(input.clone()).expect("Must be item.");
    let type_name = match impl_item.self_ty.deref() {
        Type::Path(path) => path.path.to_token_stream().to_string(),
        _ => panic!("Unable to determine good name for class"),
    };

    let mut functions = Vec::new();

    for item in &impl_item.items {
        if let Some(generated_function) = create_c_function(&attr, &impl_item, item) {
            functions.push(generated_function);
        }
    }

    let ctor = constructor(&attr, &type_name, impl_item.span());
    let dtor = deconstructor(&attr, &type_name, impl_item.span());

    let ctor: TokenStream = crate::ffi_function(proc_macro::TokenStream::new(), ctor.into()).into();
    let dtor: TokenStream = crate::ffi_function(proc_macro::TokenStream::new(), dtor.into()).into();

    let doc_line = extract_doc_lines(&impl_item.attrs).join("\n");

    quote! {
        #input

        #ctor

        #dtor

        #(
          #functions
        )*
    }
}
