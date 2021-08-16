use crate::service::function_impl::{generate_service_dtor, generate_service_method};
use darling::FromMeta;
use function_impl::MethodType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ImplItem, ItemImpl};

pub mod function_impl;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    debug: bool,

    #[darling(default)]
    error: String,

    #[darling(default)]
    prefix: String,
}

impl Attributes {
    pub fn assert_valid(&self) {}
}

pub fn ffi_service(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let attributes: Attributes = Attributes::from_list(&attr).unwrap();
    attributes.assert_valid();

    let item = syn::parse2::<ItemImpl>(input.clone()).expect("Must be item.");
    let service_type = &item.self_ty;
    let mut function_descriptors = Vec::new();

    for x in &item.items {
        if let ImplItem::Method(x) = x {
            if let Some(xx) = generate_service_method(&attributes, &item, x) {
                function_descriptors.push(xx);
            }
        }
    }

    let ffi_functions = function_descriptors.iter().map(|x| x.ffi_function_tokens.clone()).collect::<Vec<_>>();
    let ffi_dtor = generate_service_dtor(&attributes, &item);
    let ffi_method_ident = function_descriptors
        .iter()
        .filter(|x| matches!(x.method_type, MethodType::Method(_)))
        .map(|x| x.ident.clone())
        .collect::<Vec<_>>();
    let ffi_ctors = function_descriptors
        .iter()
        .filter(|x| matches!(x.method_type, MethodType::Constructor(_)))
        .map(|x| x.ident.clone())
        .collect::<Vec<_>>();

    let ffi_dtor_quote = &ffi_dtor.ffi_function_tokens;
    let ffi_dtor_ident = &ffi_dtor.ident;

    let lifetimes = item.generics.lifetimes();
    let lt = quote! { #(#lifetimes),* };

    let rval = quote! {
        #input

        #(
            #ffi_functions
        )*

        #ffi_dtor_quote

        impl <#lt> ::interoptopus::patterns::LibraryPatternInfo for #service_type {
            fn pattern_info() -> ::interoptopus::patterns::LibraryPattern {

                use ::interoptopus::lang::rust::CTypeInfo;
                use ::interoptopus::lang::rust::FunctionInfo;

                let mut methods = Vec::new();
                let mut ctors = Vec::new();

                #(
                    {
                        use #ffi_method_ident as x;
                        methods.push(x::function_info());
                    }
                )*


                #(
                    {
                        use #ffi_ctors as x;
                        ctors.push(x::function_info());
                    }
                )*

                let dtor = {
                    use #ffi_dtor_ident as x;
                    x::function_info()
                };

                let service = ::interoptopus::patterns::service::Service::new(
                    ctors, dtor, methods,
                );

                service.assert_valid();

                ::interoptopus::patterns::LibraryPattern::Service(service)
            }
        }
    };

    if attributes.debug {
        println!("{}", &rval);
    }

    rval
}

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
