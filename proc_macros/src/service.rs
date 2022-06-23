use crate::service::function_impl::{generate_service_dtor, generate_service_method};
use darling::FromMeta;
use function_impl::MethodType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ImplItem, ItemImpl, Visibility};

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

    for impl_item in &item.items {
        if let ImplItem::Method(method) = impl_item {
            match &method.vis {
                Visibility::Public(_) => {
                    if method.attrs.iter().any(|x| format!("{:?}", x).contains("ffi_service_skip")) {}
                    else {
                        if let Some(descriptor) = generate_service_method(&attributes, &item, method) {
                            function_descriptors.push(descriptor);
                        }
                    }
                }
                _ => {}
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
