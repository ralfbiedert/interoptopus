use crate::service::Attributes;
use crate::util::{extract_doc_lines, purge_lifetimes_from_type};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{FnArg, GenericParam, ImplItemFn, ItemImpl, Pat, PatType, ReturnType, Type, TypeGroup, TypeReference};

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

#[derive(Debug, FromMeta)]
pub enum OnPanic {
    FfiError,
    ReturnDefault,
    UndefinedBehavior,
}

impl Default for OnPanic {
    fn default() -> Self {
        Self::FfiError
    }
}

#[derive(Debug, FromMeta)]
pub struct AttributeMethod {
    #[darling(default)]
    ignore: bool,
    #[darling(default)]
    on_panic: OnPanic,
}

impl Default for AttributeMethod {
    fn default() -> Self {
        Self {
            ignore: false,
            on_panic: OnPanic::FfiError,
        }
    }
}

/// Inspects all attributes and determines the method type to generate.
fn method_type(function: &ImplItemFn) -> MethodType {
    let attrs = function.attrs.as_slice();

    // Methods explicitly marked a constructors
    if function.attrs.iter().any(|x| format!("{:?}", x).contains("ffi_service_ctor")) {
        let ctor_attributes = attrs
            .iter()
            .filter_map(|attribute| AttributeCtor::from_meta(&attribute.meta).ok())
            .next()
            .unwrap_or_default();

        return MethodType::Constructor(ctor_attributes);
    }

    // Methods explicitly marked a service methods
    if attrs.iter().any(|x| format!("{:?}", x).contains("ffi_service_method")) {
        let function_attributes = attrs
            .iter()
            .filter(|x| format!("{:?}", x).contains("ffi_service_method"))
            .map(|attribute| AttributeMethod::from_meta(&attribute.meta).unwrap())
            .next()
            .unwrap_or_default();

        return MethodType::Method(function_attributes);
    }

    // If the method wasn't explicitly marked ...
    match function.sig.output {
        // If it has default output type, we can get away with "return default"
        ReturnType::Default => MethodType::Method(AttributeMethod {
            ignore: false,
            on_panic: OnPanic::ReturnDefault,
        }),
        // Otherwise, use FFI error conversion.
        ReturnType::Type(_, _) => MethodType::Method(AttributeMethod {
            ignore: false,
            on_panic: OnPanic::FfiError,
        }),
    }
}

pub fn generate_service_method(attributes: &Attributes, impl_block: &ItemImpl, function: &ImplItemFn) -> Option<Descriptor> {
    let orig_fn_ident = &function.sig.ident;
    let service_type = &impl_block.self_ty;
    let service_prefix = attributes.prefered_service_name(impl_block);
    let mut generics = function.sig.generics.clone();
    let mut inputs = Vec::new();
    let mut arg_names = Vec::new();

    for lt in impl_block.generics.lifetimes() {
        generics.params.push(GenericParam::Lifetime(lt.clone()))
    }

    let ffi_fn_ident = Ident::new(&format!("{}{}", service_prefix, orig_fn_ident), function.span());
    let error_ident = Ident::new(&attributes.error, function.span());
    let without_lifetimes = purge_lifetimes_from_type(&impl_block.self_ty);
    let doc_lines = extract_doc_lines(&function.attrs);

    let span_rval = function.sig.output.span();
    let span_function = function.span();
    let span_body = function.block.span();
    let span_service_ty = impl_block.self_ty.span();

    let rval = match &function.sig.output {
        ReturnType::Default => quote_spanned!(span_rval=> ()),
        ReturnType::Type(_, x) => quote_spanned!(span_rval=> #x),
    };

    let method_type = method_type(function);

    match method_type {
        MethodType::Constructor(_) => inputs.push(quote_spanned!(span_service_ty=> context: &mut *mut #service_type)),
        MethodType::Method(method) if method.ignore => return None,
        _ => {}
    };

    let service_purged_type = Box::new(purge_lifetimes_from_type(service_type));

    let receiver_type = if matches!(method_type, MethodType::Constructor(..)) {
        service_purged_type.clone()
    } else {
        function
            .sig
            .inputs
            .first()
            .map(|fn_arg| match fn_arg {
                FnArg::Receiver(..) => service_purged_type.clone(),
                FnArg::Typed(PatType { ty, .. }) => {
                    let ty = match &**ty {
                        Type::Reference(TypeReference { elem, .. }) => elem,
                        Type::Group(TypeGroup { elem, .. }) => elem,
                        _ => ty,
                    };
                    Box::new(crate::util::purge_lifetimes_from_type(ty))
                }
            })
            .unwrap_or(service_purged_type.clone())
    };

    let type_eq_check = quote_spanned! {receiver_type.span() =>
        const _: fn() = || {
            trait TypeEq {
                type This;
            }

            impl<T> TypeEq for T {
                type This = Self;
            }

            fn assert_type_eq_all<T, U>()
            where
                T: TypeEq<This = U>
                {}

            assert_type_eq_all::<#service_purged_type, #receiver_type>();
        };
    };

    for (i, arg) in function.sig.inputs.iter().enumerate() {
        let span_arg = arg.span();
        match arg {
            FnArg::Receiver(receiver) => {
                if receiver.mutability.is_some() {
                    inputs.push(quote_spanned!(span_arg=> context: &mut #service_type));
                } else {
                    inputs.push(quote_spanned!(span_arg=> context: & #service_type));
                }

                arg_names.push(quote_spanned!(span_arg=> context));
            }
            FnArg::Typed(pat) => match pat.pat.deref() {
                Pat::Ident(x) => {
                    let i = &x.ident;
                    arg_names.push(quote_spanned!(span_arg=> #i));
                    inputs.push(quote_spanned!(span_arg=> #arg));
                }
                Pat::Wild(_) => {
                    let new_ident = Ident::new(&format!("_anon{}", i), arg.span());
                    let ty = &pat.ty;
                    arg_names.push(quote_spanned!(span_arg=> #new_ident));
                    inputs.push(quote_spanned!(span_arg=> #new_ident: #ty));
                }
                _ => panic!("Unknown pattern {:?}", pat),
            },
        }
    }

    let generated_function = match &method_type {
        MethodType::Constructor(_) => {
            quote_spanned! { span_function =>
                #[interoptopus::ffi_function]
                #[no_mangle]
                #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
                #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
                #(
                    #[doc = #doc_lines]
                )*
                pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #error_ident {

                    *context = ::std::ptr::null_mut();

                    let result_result = std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                        <#service_type>::#orig_fn_ident( #(#arg_names),* )
                    }));

                    match result_result {
                        Ok(Ok(obj)) => {
                            let boxed = ::std::boxed::Box::new(obj);
                            let raw = ::std::boxed::Box::into_raw(boxed);
                            *context = raw;

                            <#error_ident as ::interoptopus::patterns::result::FFIError>::SUCCESS
                        }

                        Ok(Err(e)) => {
                            ::interoptopus::util::log_error(|| format!("Error in ({}): {:?}", stringify!(#ffi_fn_ident), e));
                            e.into()
                        }

                        Err(e) => {
                            ::interoptopus::util::log_error(|| format!("Panic in ({}): {}", stringify!(#ffi_fn_ident), ::interoptopus::patterns::result::get_panic_message(e.as_ref())));
                            <#error_ident as ::interoptopus::patterns::result::FFIError>::PANIC
                        }
                    }
                }
            }
        }
        MethodType::Method(x) => match x.on_panic {
            OnPanic::ReturnDefault => {
                quote_spanned! { span_function =>
                    #type_eq_check
                    #[interoptopus::ffi_function]
                    #[no_mangle]
                    #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
                    #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
                    #(
                        #[doc = #doc_lines]
                    )*
                    pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #rval {
                        let result_result = ::std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            // Make sure we only have a FnOnce closure and prevent lifetime errors.
                            #(
                                let #arg_names = #arg_names;
                            )*
                            <#without_lifetimes>::#orig_fn_ident( #(#arg_names),* )
                        }));

                        match result_result {
                            Ok(x) => x,
                            Err(e) => {
                                ::interoptopus::util::log_error(|| format!("Panic in ({}): {}", stringify!(#ffi_fn_ident), ::interoptopus::patterns::result::get_panic_message(e.as_ref())));
                                <#rval>::default()
                            }
                        }
                    }
                }
            }
            OnPanic::UndefinedBehavior => {
                quote_spanned! { span_function =>
                    #type_eq_check
                    #[interoptopus::ffi_function]
                    #[no_mangle]
                    #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
                    #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
                    #(
                        #[doc = #doc_lines]
                    )*
                    pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #rval {
                        <#without_lifetimes>::#orig_fn_ident( #(#arg_names),* )
                    }
                }
            }
            OnPanic::FfiError => {
                let block = quote_spanned! { span_body =>
                    <#without_lifetimes>::#orig_fn_ident( #(#arg_names),* )
                };

                quote_spanned! { span_function =>
                    #type_eq_check
                    #[interoptopus::ffi_function]
                    #[no_mangle]
                    #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
                    #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
                    #(
                        #[doc = #doc_lines]
                    )*
                    pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #error_ident {
                        ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(move || {
                            #block
                        }, stringify!(#ffi_fn_ident))
                    }
                }
            }
        },
        MethodType::Destructor => panic!("Must not happen."),
    };

    Some(Descriptor {
        ffi_function_tokens: generated_function,
        ident: ffi_fn_ident,
        method_type,
    })
}

pub fn generate_service_dtor(attributes: &Attributes, impl_block: &ItemImpl) -> Descriptor {
    let service_prefix = attributes.prefered_service_name(impl_block);
    let ffi_fn_ident = Ident::new(&format!("{}destroy", service_prefix), impl_block.span());
    let error_ident = Ident::new(&attributes.error, impl_block.span());
    let without_lifetimes = purge_lifetimes_from_type(&impl_block.self_ty);

    let span_service_ty = impl_block.self_ty.span();

    let generated_function = quote_spanned! {span_service_ty=>
        /// Destroys the given instance.
        ///
        /// # Safety
        ///
        /// The passed parameter MUST have been created with the corresponding init function;
        /// passing any other value results in undefined behavior.
        #[interoptopus::ffi_function]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn, unused_unsafe)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
        #[no_mangle]
        pub unsafe extern "C" fn #ffi_fn_ident(context: &mut *mut #without_lifetimes) -> #error_ident {
            // Checks the _contained_ pointer is not null, which usually means service was not initialized.
            if context.is_null() {
                return <#error_ident as ::interoptopus::patterns::result::FFIError>::NULL;
            }

            let result_result = ::std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                unsafe { drop(::std::boxed::Box::from_raw(*context)) };
            }));

            *context = ::std::ptr::null_mut();

            match result_result {
                Ok(_) => <#error_ident as ::interoptopus::patterns::result::FFIError>::SUCCESS,
                Err(e) => {
                    ::interoptopus::util::log_error(|| format!("Panic in ({}): {}", stringify!(#ffi_fn_ident), ::interoptopus::patterns::result::get_panic_message(e.as_ref())));
                    <#error_ident as ::interoptopus::patterns::result::FFIError>::PANIC
                }
            }
        }
    };

    Descriptor {
        ffi_function_tokens: generated_function,
        ident: ffi_fn_ident,
        method_type: MethodType::Destructor,
    }
}
