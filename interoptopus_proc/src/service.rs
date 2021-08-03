use crate::service::function_impl::generate_service_method;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, FnArg, GenericArgument, GenericParam, ImplItem, ItemFn, ItemImpl, Pat, PathArguments, ReturnType, Signature, Type};

pub mod function_impl;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    debug: bool,

    #[darling(default)]
    error: String,
}

impl Attributes {
    pub fn assert_valid(&self) {}
}

pub fn ffi_service(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let attributes: Attributes = Attributes::from_list(&attr).unwrap();
    attributes.assert_valid();

    let item = syn::parse2::<ItemImpl>(input.clone()).expect("Must be item.");
    let mut service_type = &item.self_ty;
    let mut function_descriptors = Vec::new();

    for x in &item.items {
        match x {
            ImplItem::Method(x) => {
                if let Some(xx) = generate_service_method(&attributes, &item, x) {
                    function_descriptors.push(xx);
                }
            }
            _ => {}
        }
    }

    let ffi_functions = function_descriptors.iter().map(|x| x.ffi_function_tokens.clone()).collect::<Vec<_>>();

    let rval = quote! {
        #input

        #(
            #ffi_functions
        )*

        impl ::interoptopus::patterns::service::ServiceInfo for #service_type {
            fn service_info() -> ::interoptopus::patterns::service::Service {

                todo!()
            }
        }
    };

    if attributes.debug {
        println!("{}", &rval);
    }

    rval
}
