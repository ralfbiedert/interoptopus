use crate::service::Attributes;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{Attribute, AttributeArgs, FnArg, ImplItemMethod, ItemImpl, Pat, PatType, ReturnType, Type};

pub struct Descriptor {
    pub ffi_function_tokens: TokenStream,
}

#[derive(Debug)]
enum MethodType {
    Constructor(AttributeCtor),
    Method(AttributeMethod),
}

#[derive(Debug, Default, FromMeta)]
pub struct AttributeCtor {}

#[derive(Debug, Default, FromMeta)]
pub struct AttributeMethod {
    #[darling(default)]
    direct: bool,
}

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
    let generics = function.sig.generics.clone();
    let mut inputs = Vec::new();
    let mut arg_names = Vec::new();

    let ffi_fn_ident = Ident::new(&format!("ffi_xxx_{}", orig_fn_ident.to_string()), function.span());
    let error_ident = Ident::new(&attributes.error, function.span());

    let rval = match &function.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, x) => quote! { #x },
    };

    let method_type = method_type(&function.attrs);

    // Constructor needs extra arg for ptr
    if let MethodType::Constructor(_) = &method_type {
        inputs.push(quote! { this: &mut *mut #service_type });
    }

    for (i, arg) in function.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(receiver) => {
                if receiver.mutability.is_some() {
                    inputs.push(quote! { this: &mut #service_type });
                } else {
                    inputs.push(quote! { this: & #service_type });
                }

                arg_names.push(quote! { this });
            }
            FnArg::Typed(pat) => match pat.pat.deref() {
                Pat::Ident(x) => {
                    let i = &x.ident;
                    arg_names.push(quote! { #i });
                    inputs.push(quote! { #arg });
                }
                Pat::Wild(x) => {
                    let new_ident = Ident::new(&*format!("_anon{}", i), arg.span());
                    let ty = &pat.ty;
                    arg_names.push(quote! { #new_ident });
                    inputs.push(quote! { #new_ident: #ty });
                }
                _ => panic!("Unknown pattern {:?}", pat),
            },
        }
    }

    let generated_function = match method_type {
        MethodType::Constructor(x) => {
            quote! {
                #[no_mangle]
                pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #error_ident {

                    let result_result = std::panic::catch_unwind(|| {
                        <#service_type>::#orig_fn_ident( #(#arg_names),* )
                    });

                    match result_result {
                        Ok(Ok(obj)) => {
                            let boxed = Box::new(obj);
                            let raw = Box::into_raw(boxed);
                            *this = raw;

                            <#error_ident as ::interoptopus::patterns::success_enum::Success>::SUCCESS
                        }

                        Ok(x) => {
                            ::interoptopus::util::log_error(|| "xxxxxx");
                            x.into()
                        }

                        Err(_) => {
                            ::interoptopus::util::log_error(|| "xxxxxx");
                            <#error_ident as ::interoptopus::patterns::success_enum::Success>::PANIC
                        }
                    }
                }
            }
        }
        MethodType::Method(x) => {
            if x.direct {
                quote! {
                    #[no_mangle]
                    pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #rval {
                        let result_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            <#service_type>::#orig_fn_ident( #(#arg_names),* )
                        }));

                        match result_result {
                            Ok(x) => {
                                x
                            }
                            Err(e) => {
                                ::interoptopus::util::log_error(|| "xxxxxx");
                                <#rval>::default()
                            }
                        }
                    }
                }
            } else {
                quote! {
                    #[no_mangle]
                    pub extern "C" fn #ffi_fn_ident #generics( #(#inputs),* ) -> #error_ident {

                        let result_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            <#service_type>::#orig_fn_ident( #(#arg_names),* )
                        }));

                        match result_result {
                            Ok(Ok(_)) => <FFIError as interoptopus::patterns::success_enum::Success>::SUCCESS,
                            Ok(x) => {
                                ::interoptopus::util::log_error(|| "xxxxxx");
                                x.into()
                            }
                            Err(e) => {
                                ::interoptopus::util::log_error(|| "xxxxxx");
                                <#error_ident as ::interoptopus::patterns::success_enum::Success>::PANIC
                            }
                        }
                    }
                }
            }
        }
    };

    Some(Descriptor {
        ffi_function_tokens: generated_function,
    })
}

//
//
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_create(context_ptr: Option<&mut *mut SimpleService>, x: u32) -> FFIError {
//     if let Some(context) = context_ptr {
//         let result_result = std::panic::catch_unwind(|| {
//             <SimpleService>::new_with(x)
//         });
//
//         match result_result {
//             Ok(Ok(obj)) => {
//                 let boxed = Box::new(obj);
//                 let raw = Box::into_raw(boxed);
//                 *context = raw;
//
//                 <FFIError as ::interoptopus::patterns::success_enum::Success>::SUCCESS
//             }
//             Ok(x) => {
//                 x.into()
//             }
//
//             Err(_) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Error or panic in function `{}`", stringify!( simple_service_create )));
//                     res
//                 });
//                 <FFIError as ::interoptopus::patterns::success_enum::Success>::PANIC
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_create )));
//             res
//         });
//         <FFIError as interoptopus::patterns::success_enum::Success>::NULL
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_destroy(context_ptr: Option<&mut *mut SimpleService>) -> FFIError {
//     if let Some(context) = context_ptr {
//         {
//             unsafe { Box::from_raw(*context) };
//         }
//
//         *context = std::ptr::null_mut();
//
//         <FFIError as interoptopus::patterns::success_enum::Success>::SUCCESS
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_destroy )));
//             res
//         });
//         <FFIError as interoptopus::patterns::success_enum::Success>::NULL
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_result(context_ptr: Option<&mut SimpleService>, x: u32) -> FFIError {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_result
//                 (context, x)
//         })) {
//             Ok(Ok(_)) => <FFIError as interoptopus::patterns::success_enum::Success>::SUCCESS,
//             Ok(Err(e)) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Error in function `{}`: {}", stringify!( simple_service_result ), e.to_string()));
//                     res
//                 });
//                 <FFIError>::from(Result::<(), _>::Err(e))
//             }
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic in function `{}`", stringify!( simple_service_result )));
//                     res
//                 });
//                 <FFIError as interoptopus::patterns::service::FailureDefault>::failure_default()
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_result )));
//             res
//         });
//         <FFIError as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_value(context_ptr: Option<&mut SimpleService>, x: u32) -> u32 {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_value(context, x)
//         })) {
//             Ok(rval) => rval.into(),
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic or error in function `{}`", stringify!( simple_service_value )));
//                     res
//                 });
//                 return <u32 as interoptopus::patterns::service::FailureDefault>::failure_default();
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_value )));
//             res
//         });
//         <u32 as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_mut_self(context_ptr: Option<&mut SimpleService>, slice: FFISlice<u8>) -> u8 {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_mut_self(context, slice)
//         })) {
//             Ok(rval) => rval.into(),
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic or error in function `{}`", stringify!( simple_service_mut_self )));
//                     res
//                 });
//                 return <u8 as interoptopus::patterns::service::FailureDefault>::failure_default();
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_mut_self )));
//             res
//         });
//         <u8 as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_mut_self_void(context_ptr: Option<&mut SimpleService>, slice: FFISlice<FFIBool>) -> () {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_mut_self_void(context, slice)
//         })) {
//             Ok(rval) => rval.into(),
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic or error in function `{}`", stringify!( simple_service_mut_self_void )));
//                     res
//                 });
//                 return <() as interoptopus::patterns::service::FailureDefault>::failure_default();
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_mut_self_void )));
//             res
//         });
//         <() as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_mut_self_ref(context_ptr: Option<&mut SimpleService>, x: &u8, _y: &mut u8) -> u8 {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_mut_self_ref(context, x, _y)
//         })) {
//             Ok(rval) => rval.into(),
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic or error in function `{}`", stringify!( simple_service_mut_self_ref )));
//                     res
//                 });
//                 return <u8 as interoptopus::patterns::service::FailureDefault>::failure_default();
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_mut_self_ref )));
//             res
//         });
//         <u8 as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_mut_self_ref_slice(context_ptr: Option<&mut SimpleService>, x: &u8, _y: &mut u8, _slice: FFISlice<u8>) -> u8 {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_mut_self_ref_slice(context, x, _y, _slice)
//         })) {
//             Ok(rval) => rval.into(),
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic or error in function `{}`", stringify!( simple_service_mut_self_ref_slice )));
//                     res
//                 });
//                 return <u8 as interoptopus::patterns::service::FailureDefault>::failure_default();
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_mut_self_ref_slice )));
//             res
//         });
//         <u8 as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_mut_self_ref_slice_limited<'a, 'b, >(context_ptr: Option<&mut SimpleService>, x: &u8, _y: &mut u8, _slice: FFISlice<'a, u8>, _slice2: FFISlice<'b, u8>) -> u8 {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_mut_self_ref_slice_limited(context, x, _y, _slice, _slice2)
//         })) {
//             Ok(rval) => rval.into(),
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic or error in function `{}`", stringify!( simple_service_mut_self_ref_slice_limited )));
//                     res
//                 });
//                 return <u8 as interoptopus::patterns::service::FailureDefault>::failure_default();
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_mut_self_ref_slice_limited )));
//             res
//         });
//         <u8 as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_mut_self_ffi_error(context_ptr: Option<&mut SimpleService>, slice: FFISliceMut<u8>) -> FFIError {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_mut_self_ffi_error(context, slice)
//         })) {
//             Ok(rval) => rval.into(),
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic or error in function `{}`", stringify!( simple_service_mut_self_ffi_error )));
//                     res
//                 });
//                 return <FFIError as interoptopus::patterns::service::FailureDefault>::failure_default();
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_mut_self_ffi_error )));
//             res
//         });
//         <FFIError as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// #[interoptopus::ffi_function]
// #[no_mangle]
// pub extern "C" fn simple_service_void(context_ptr: Option<&SimpleService> ) -> () {
//     if let Some(context) = context_ptr {
//         match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//             <SimpleService>::method_void
//                 (context )
//         })) {
//             Ok(rval) => rval.into(),
//             Err(e) => {
//                 ::interoptopus::util::log_error(|| {
//                     let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Panic or error in function `{}`", stringify!( simple_service_void )));
//                     res
//                 });
//                 return <() as interoptopus::patterns::service::FailureDefault>::failure_default();
//             }
//         }
//     } else {
//         ::interoptopus::util::log_error(|| {
//             let res = ::alloc::fmt::format(IntellijRustDollarCrate::__export::format_args!("Null pointer in function `{}`", stringify!( simple_service_void )));
//             res
//         });
//         <() as interoptopus::patterns::service::FailureDefault>::failure_default()
//     }
// }
// pub(crate) fn simple_service_pattern() -> interoptopus::patterns::service::Service {
//     use interoptopus::lang::rust::CTypeInfo;
//     use interoptopus::lang::rust::FunctionInfo;
//
//     let mut methods = Vec::new();
//
//     {
//         {
//             use simple_service_result
//             as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_value    as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_mut_self    as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_mut_self_void    as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_mut_self_ref    as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_mut_self_ref_slice    as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_mut_self_ref_slice_limited    as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_mut_self_ffi_error    as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_void
//             as x;
//             methods.push(x::function_info());
//         }
//         {
//             use simple_service_extra_method
//
//             as x;
//             methods.push(x::function_info());
//         }
//     }
//
//     let ctor = {
//         use simple_service_create    as x;
//         x::function_info()
//     };
//
//     let dtor = {
//         use simple_service_destroy    as x;
//         x::function_info()
//     };
//
//     let rval = interoptopus::patterns::service::Service::new(
//         ctor, dtor, methods,
//     );
//
//     rval.assert_valid();
//     rval
// }
