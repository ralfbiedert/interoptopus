use crate::service::Attributes;
use crate::util::{extract_doc_lines, purge_lifetimes_from_type};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{Attribute, FnArg, GenericParam, ImplItemMethod, ItemImpl, Pat, ReturnType};

pub struct Descriptor {
    pub ffi_function_tokens: TokenStream,
    pub ident: Ident,
    pub method_type: MethodType,
}

#[derive(Debug)]
pub enum MethodType {
    Constructor(AttributeCtor),
    Method(AttributeMethod),
    Destructor,
}

#[derive(Debug, Default, FromMeta)]
pub struct AttributeCtor {}

#[derive(Debug, Default, FromMeta)]
pub struct AttributeMethod {
    #[darling(default)]
    direct: bool,
}

/// Inspects all attributes and determines the method type to generate.
fn method_type(attrs: &[Attribute]) -> MethodType {
    if attrs.iter().any(|x| format!("{:?}", x).contains("ffi_service_ctor")) {
        let ctor_attributes = attrs
            .iter()
            .filter_map(|attribute| {
                let meta = attribute.parse_meta().ok()?;
                AttributeCtor::from_meta(&meta).ok()
            })
            .next()
            .unwrap_or_default();

        MethodType::Constructor(ctor_attributes)
    } else {
        let function_attributes = attrs
            .iter()
            .filter(|x| format!("{:?}", x).contains("ffi_service_method"))
            .filter_map(|attribute| {
                let meta = attribute.parse_meta().ok()?;
                AttributeMethod::from_meta(&meta).ok()
            })
            .next()
            .unwrap_or_default();

        MethodType::Method(function_attributes)
    }
}

pub fn generate_service_method(attributes: &Attributes, impl_block: &ItemImpl, function: &ImplItemMethod) -> Option<Descriptor> {
    let orig_fn_ident = &function.sig.ident;
    let service_type = &impl_block.self_ty;
    let mut generics = function.sig.generics.clone();
    let mut inputs = Vec::new();
    let mut arg_names = Vec::new();

    for lt in impl_block.generics.lifetimes() {
        generics.params.push(GenericParam::Lifetime(lt.clone()))
    }

    let ffi_fn_ident = Ident::new(&format!("{}{}", attributes.prefix, orig_fn_ident.to_string()), function.span());
    let error_ident = Ident::new(&attributes.error, function.span());
    let without_lifetimes = purge_lifetimes_from_type(&*impl_block.self_ty);
    let doc_lines = extract_doc_lines(&function.attrs);

    let rval = match &function.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, x) => quote! { #x },
    };

    let method_type = method_type(&function.attrs);

    // Constructor needs extra arg for ptr
    if let MethodType::Constructor(_) = &method_type {
        inputs.push(quote! { context: &mut *mut #service_type });
    }

    for (i, arg) in function.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(receiver) => {
                if receiver.mutability.is_some() {
                    inputs.push(quote! { context: &mut #service_type });
                } else {
                    inputs.push(quote! { context: & #service_type });
                }

                arg_names.push(quote! { context });
            }
            FnArg::Typed(pat) => match pat.pat.deref() {
                Pat::Ident(x) => {
                    let i = &x.ident;
                    arg_names.push(quote! { #i });
                    inputs.push(quote! { #arg });
                }
                Pat::Wild(_) => {
                    let new_ident = Ident::new(&*format!("_anon{}", i), arg.span());
                    let ty = &pat.ty;
                    arg_names.push(quote! { #new_ident });
                    inputs.push(quote! { #new_ident: #ty });
                }
                _ => panic!("Unknown pattern {:?}", pat),
            },
        }
    }

    let generated_function = match &method_type {
        MethodType::Constructor(_) => {
            quote! {
                #[interoptopus::ffi_function]
                #[no_mangle]
                #(
                    #[doc = #doc_lines]
                )*
                pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #error_ident {

                    let result_result = std::panic::catch_unwind(|| {
                        <#service_type>::#orig_fn_ident( #(#arg_names),* )
                    });

                    match result_result {
                        Ok(Ok(obj)) => {
                            let boxed = Box::new(obj);
                            let raw = Box::into_raw(boxed);
                            *context = raw;

                            <#error_ident as ::interoptopus::patterns::result::FFIError>::SUCCESS
                        }

                        Ok(Err(e)) => {
                            ::interoptopus::util::log_error(|| format!("Error in ({}): {:?}", stringify!(#ffi_fn_ident), e));
                            e.into()
                        }

                        Err(e) => {
                            ::interoptopus::util::log_error(|| format!("Panic in ({}): {:?}", stringify!(#ffi_fn_ident), e));
                            <#error_ident as ::interoptopus::patterns::result::FFIError>::PANIC
                        }
                    }
                }
            }
        }
        MethodType::Method(x) => {
            if x.direct {
                quote! {
                    #[interoptopus::ffi_function]
                    #[no_mangle]
                    #(
                        #[doc = #doc_lines]
                    )*
                    pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #rval {
                        let result_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            <#without_lifetimes>::#orig_fn_ident( #(#arg_names),* )
                        }));

                        match result_result {
                            Ok(x) => x,
                            Err(e) => {
                                ::interoptopus::util::log_error(|| format!("Panic in ({}): {:?}", stringify!(#ffi_fn_ident), e));
                                <#rval>::default()
                            }
                        }
                    }
                }
            } else {
                quote! {
                    #[interoptopus::ffi_function]
                    #[no_mangle]
                    #(
                        #[doc = #doc_lines]
                    )*
                    pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #error_ident {
                        ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(|| {
                            <#without_lifetimes>::#orig_fn_ident( #(#arg_names),* )
                        }, stringify!(#ffi_fn_ident))
                    }
                }
            }
        }
        MethodType::Destructor => panic!("Must not happen."),
    };

    Some(Descriptor {
        ffi_function_tokens: generated_function,
        ident: ffi_fn_ident,
        method_type,
    })
}

pub fn generate_service_dtor(attributes: &Attributes, impl_block: &ItemImpl) -> Descriptor {
    let ffi_fn_ident = Ident::new(&format!("{}simple_service_destroy", attributes.prefix), impl_block.span());
    let error_ident = Ident::new(&attributes.error, impl_block.span());
    let without_lifetimes = purge_lifetimes_from_type(&*impl_block.self_ty);

    let generated_function = quote! {
        /// Destroys the given instance.
        ///
        /// # Safety
        ///
        /// The passed parameter MUST have been created with the corresponding init function;
        /// passing any other value results in undefined behavior.
        #[interoptopus::ffi_function]
        #[no_mangle]
        pub unsafe extern "C" fn #ffi_fn_ident(context: &mut *mut #without_lifetimes) -> #error_ident {
            {
                unsafe { Box::from_raw(*context) };
            }

            *context = ::std::ptr::null_mut();

            <#error_ident as ::interoptopus::patterns::result::FFIError>::SUCCESS
        }
    };

    Descriptor {
        ffi_function_tokens: generated_function,
        ident: ffi_fn_ident,
        method_type: MethodType::Destructor,
    }
}
