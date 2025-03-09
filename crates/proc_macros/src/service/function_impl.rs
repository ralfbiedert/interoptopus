use crate::service::Attributes;
use crate::util::{extract_doc_lines, purge_lifetimes_from_type};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{FnArg, GenericParam, ImplItem, ImplItemFn, ItemImpl, Pat, ReturnType};

pub struct Descriptor {
    pub ffi_function_tokens: TokenStream,
    pub ident: Ident,
    pub method_type: MethodType,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum MethodType {
    Constructor,
    MethodAsync(AttributeMethodAsync),
    MethodSync(AttributeMethodSync),
    Destructor,
}

#[derive(Debug, FromMeta)]
pub enum OnPanic {
    FfiError,
    ReturnDefault,
    Abort,
}

impl Default for OnPanic {
    fn default() -> Self {
        Self::FfiError
    }
}

#[derive(Default, Debug, FromMeta)]
pub struct AttributeMethodSync {
    #[darling(default)]
    ignore: bool,
    #[darling(default)]
    on_panic: OnPanic,
}

#[derive(Default, Debug, FromMeta)]
#[allow(dead_code)]
pub struct AttributeMethodAsync {
    #[darling(default)]
    ignore: bool,
}

/// Inspects all attributes and determines the method type to generate.
#[allow(clippy::match_like_matches_macro)]
#[allow(clippy::match_wildcard_for_single_variants)]
fn method_type(function: &ImplItemFn) -> MethodType {
    let attrs = function.attrs.as_slice();

    // To be a ctor the function must
    // - not take &self
    // - and not be `async`
    let is_ctor = function.sig.inputs.iter().next().is_none_or(|x| match x {
        FnArg::Typed(_) if function.sig.asyncness.is_none() => true,
        _ => false,
    });

    if is_ctor {
        return MethodType::Constructor;
    }

    // Methods that have an `async fn` are always async.
    if function.sig.asyncness.is_some() {
        let function_attributes = attrs
            .iter()
            .filter(|x| format!("{x:?}").contains("ffi_service_method"))
            .map(|attribute| AttributeMethodAsync::from_meta(&attribute.meta).unwrap())
            .next()
            .unwrap_or_default();

        return MethodType::MethodAsync(function_attributes);
    }

    // Methods explicitly marked a service methods
    if attrs.iter().any(|x| format!("{x:?}").contains("ffi_service_method")) {
        let function_attributes = attrs
            .iter()
            .filter(|x| format!("{x:?}").contains("ffi_service_method"))
            .map(|attribute| AttributeMethodSync::from_meta(&attribute.meta).unwrap())
            .next()
            .unwrap_or_default();

        return MethodType::MethodSync(function_attributes);
    }

    // If the method wasn't explicitly marked ...
    match function.sig.output {
        // If it has default output type, we can get away with "return default"
        ReturnType::Default => MethodType::MethodSync(AttributeMethodSync { ignore: false, on_panic: OnPanic::ReturnDefault }),
        // Otherwise, use FFI error conversion.
        ReturnType::Type(_, _) => MethodType::MethodSync(AttributeMethodSync { ignore: false, on_panic: OnPanic::FfiError }),
    }
}

#[allow(
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::match_same_arms,
    clippy::explicit_deref_methods,
    clippy::redundant_clone,
    clippy::bind_instead_of_map,
    clippy::forget_non_drop,
    clippy::or_fun_call
)]
pub fn generate_service_method(attributes: &Attributes, impl_block: &ItemImpl, function: &ImplItemFn) -> Option<Descriptor> {
    let orig_fn_ident = &function.sig.ident;
    let service_type = &impl_block.self_ty;
    let service_prefix = attributes.prefered_service_name(impl_block);
    let has_async = has_async_methods(impl_block);
    let mut generics = function.sig.generics.clone();
    let mut inputs = Vec::new();
    let mut arg_names = Vec::new();
    let mut arg_types = Vec::new();

    for lt in impl_block.generics.lifetimes() {
        generics.params.push(GenericParam::Lifetime(lt.clone()));
    }

    let ffi_fn_ident = Ident::new(&format!("{service_prefix}{orig_fn_ident}"), function.span());
    let without_lifetimes = purge_lifetimes_from_type(&impl_block.self_ty);
    let doc_lines = extract_doc_lines(&function.attrs);

    let span_rval = function.sig.output.span();
    let span_function = function.span();
    let span_body = function.block.span();
    let span_service_ty = impl_block.self_ty.span();

    // Determines what the return type is, `()` or `X`
    let rval = match &function.sig.output {
        ReturnType::Default => quote_spanned!(span_rval=> ()),
        ReturnType::Type(_, x) => quote_spanned!(span_rval=> #x),
    };

    // Type of method we process (Constructor, Async, Method, Destructor)
    let method_type = method_type(function);

    match method_type {
        MethodType::MethodSync(method) if method.ignore => return None,
        _ => {}
    }

    for (i, arg) in function.sig.inputs.iter().enumerate() {
        let span_arg = arg.span();
        match arg {
            FnArg::Receiver(receiver) => {
                if receiver.mutability.is_some() {
                    inputs.push(quote_spanned!(span_arg=> __context: &mut #service_type));
                } else {
                    inputs.push(quote_spanned!(span_arg=> __context: & #service_type));
                }

                arg_names.push(quote_spanned!(span_arg=> __context));
                arg_types.push(quote_spanned!(span_arg=> Self));
            }
            FnArg::Typed(pat) => match pat.pat.deref() {
                Pat::Ident(x) => {
                    let ty = &pat.ty;
                    let ident = &x.ident;
                    arg_types.push(quote_spanned!(span_arg=> #ty));

                    // If this is the first parameter and we have `async`, the method must have
                    // requested an `Arc<T>`. In that case we generate an FFI method asking for `&T`
                    // and then convert it to `Arc<T>` internally.
                    if i == 0 && has_async && !matches!(method_type, MethodType::Constructor) {
                        arg_names.push(quote_spanned!(span_arg=> __context));
                        inputs.push(quote_spanned!(span_arg=> __context: & #service_type));
                        continue;
                    }

                    arg_names.push(quote_spanned!(span_arg=> #ident));
                    inputs.push(quote_spanned!(span_arg=> #arg));
                }
                Pat::Wild(_) => {
                    let new_ident = Ident::new(&format!("_anon{i}"), arg.span());
                    let ty = &pat.ty;
                    arg_names.push(quote_spanned!(span_arg=> #new_ident));
                    arg_types.push(quote_spanned!(span_arg=> #ty));
                    inputs.push(quote_spanned!(span_arg=> #new_ident: #ty));
                }
                _ => panic!("Unknown pattern {pat:?}"),
            },
        }
    }

    let method_attributes = quote_spanned! {span_service_ty =>
        #[::interoptopus::ffi_function]
        #[unsafe(no_mangle)]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals, clippy::forget_non_drop, clippy::useless_conversion)]
        #(
            #[doc = #doc_lines]
        )*
    };

    let generated_function = match &method_type {
        MethodType::Constructor => {
            let object_construction = if has_async {
                quote_spanned! { span_service_ty =>
                    let __boxed = ::std::sync::Arc::new(__res.unwrap());
                    let __raw = ::std::sync::Arc::into_raw(__boxed);
                }
            } else {
                quote_spanned! { span_service_ty =>
                    let __boxed = ::std::boxed::Box::new(__res.unwrap());
                    let __raw = ::std::boxed::Box::into_raw(__boxed);
                }
            };

            let ctor_result = quote_spanned! {span_rval => <<#service_type as ::interoptopus::patterns::service::ServiceInfo>::CtorResult as ::interoptopus::patterns::result::FFIResultAsPtr>::AsPtr };

            quote_spanned! { span_function =>
                #method_attributes
                pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #ctor_result {
                    let __result_result = std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                        <#service_type>::#orig_fn_ident( #(#arg_names),* )
                    }));

                    match __result_result {
                        Ok(__res) if __res.is_ok() => {
                            #object_construction
                            #ctor_result::ok(__raw)
                        }
                        Ok(__res) => {
                            let __e = __res.unwrap_err();
                            ::interoptopus::util::log_error(|| format!("Error in ({}): {:?}", stringify!(service_basic_new), __e));
                            #ctor_result::err(*__e)
                        }
                        Err(__e) => {
                            ::interoptopus::util::log_error(|| format!("Panic in ({}): {}", stringify!(#ffi_fn_ident), ::interoptopus::patterns::result::get_panic_message(__e.as_ref())));
                            #ctor_result::panic()
                        }
                    }
                }
            }
        }
        MethodType::MethodSync(x) => {
            match x.on_panic {
                OnPanic::ReturnDefault => {
                    quote_spanned! { span_function =>
                        #method_attributes
                        pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #rval {
                            let __result_result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                                // Make sure we only have a FnOnce closure and prevent lifetime errors.
                                #(
                                    let #arg_names = #arg_names;
                                )*
                                <#without_lifetimes>::#orig_fn_ident( #(#arg_names),* )
                            }));

                            match __result_result {
                                Ok(__x) => __x,
                                Err(__e) => {
                                    ::interoptopus::util::log_error(|| format!("Panic in ({}): {}", stringify!(#ffi_fn_ident), ::interoptopus::patterns::result::get_panic_message(__e.as_ref())));
                                    <#rval>::default()
                                }
                            }
                        }
                    }
                }
                OnPanic::Abort => {
                    quote_spanned! { span_function =>
                        #method_attributes
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
                        #method_attributes
                        pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #rval {
                            #block
                        }
                    }
                }
            }
        }
        MethodType::Destructor => panic!("Must not happen."),
        MethodType::MethodAsync(_) => {
            let first = arg_types.first().unwrap();
            let block = quote_spanned! { span_body =>
                // We need &T down below to invoke spawn but override the name, so let's save
                // it here
                let __this = __context;

                // We must convert the element pointer into an Arc, then clone that Arc,
                // but not drop the original one (which is the responsibility of the
                // destructor)
                let __arc_restored = unsafe { ::std::sync::Arc::from_raw(__context) };
                let __context = ::std::sync::Arc::clone(&__arc_restored);
                let _ = ::std::sync::Arc::into_raw(__arc_restored);

                let __async_fn = async move |__tlcontext| {
                    let __context = <#first as ::interoptopus::patterns::asynk::AsyncProxy<_, _>>::new(__context, __tlcontext);
                    let __rval = <#without_lifetimes>::#orig_fn_ident( #(#arg_names),* ).await.into();
                    __async_callback.call(&__rval);
                    // We actually want move semantics for rval for types like `Utf8Strings` that
                    // should be owned by the FFI side now. We therefore forget it here since
                    // the caller must have moved it out by now.
                    ::std::mem::forget(__rval);
                };

                <#without_lifetimes>::spawn(__this, __async_fn);
                <#rval as ::interoptopus::patterns::result::FFIResultAsUnitT>::AsUnitT::ok(())
            };

            quote_spanned! { span_function =>
                #method_attributes

                pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),*, __async_callback: ::interoptopus::patterns::asynk::AsyncCallback<#rval>) -> <#rval as ::interoptopus::patterns::result::FFIResultAsUnitT>::AsUnitT {
                    #block
                }
            }
        }
    };

    Some(Descriptor { ffi_function_tokens: generated_function, ident: ffi_fn_ident, method_type })
}

pub fn generate_service_dtor(attributes: &Attributes, impl_block: &ItemImpl) -> Descriptor {
    let service_prefix = attributes.prefered_service_name(impl_block);
    let ffi_fn_ident = Ident::new(&format!("{service_prefix}destroy"), impl_block.span());
    let without_lifetimes = purge_lifetimes_from_type(&impl_block.self_ty);
    let has_async = has_async_methods(impl_block);

    let span_service_ty = impl_block.self_ty.span();

    let ptr_type = if has_async {
        quote_spanned!(span_service_ty => *const #without_lifetimes)
    } else {
        quote_spanned!(span_service_ty => *mut #without_lifetimes)
    };

    let ctor_result = quote_spanned! {span_service_ty => <<#without_lifetimes as ::interoptopus::patterns::service::ServiceInfo>::CtorResult as ::interoptopus::patterns::result::FFIResultAsPtr>::AsPtr };

    let object_deconstruction = if has_async {
        quote_spanned! { span_service_ty =>
            unsafe { drop(::std::sync::Arc::from_raw(__context)) };
        }
    } else {
        quote_spanned! { span_service_ty =>
            unsafe { drop(::std::boxed::Box::from_raw(__context)) };
        }
    };

    let generated_function = quote_spanned! {span_service_ty =>
        /// Destroys the given instance.
        ///
        /// # Safety
        ///
        /// The passed parameter MUST have been created with the corresponding init function;
        /// passing any other value results in undefined behavior.
        #[::interoptopus::ffi_function]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn, unused_unsafe)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #ffi_fn_ident(__context: #ptr_type) -> #ctor_result {
            // Checks the _contained_ pointer is not null, which usually means service was not initialized.
            if __context.is_null() {
                return #ctor_result::null();
            }

            let __result_result = ::std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #object_deconstruction
            }));

            match __result_result {
                Ok(_) => #ctor_result::ok(::std::ptr::null()),
                Err(__e) => {
                    ::interoptopus::util::log_error(|| format!("Panic in ({}): {}", stringify!(#ffi_fn_ident), ::interoptopus::patterns::result::get_panic_message(__e.as_ref())));
                    #ctor_result::panic()
                }
            }
        }
    };

    Descriptor { ffi_function_tokens: generated_function, ident: ffi_fn_ident, method_type: MethodType::Destructor }
}

/// Checks if the impl block as an `async fn`.
fn has_async_methods(impl_block: &ItemImpl) -> bool {
    impl_block.items.iter().any(|x| matches!(x, ImplItem::Fn(x) if x.sig.asyncness.is_some()))
}
