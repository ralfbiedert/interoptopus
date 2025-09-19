use crate::macros::darling_parse;
use crate::service::function_impl::{generate_service_dtor, generate_service_method};
use crate::util::{get_type_name, pascal_to_snake_case, prettyprint_tokenstream};
use darling::FromMeta;
use function_impl::MethodType;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{ImplItem, ItemImpl, ReturnType, Visibility};

pub mod function_impl;

#[derive(Debug, FromMeta)]
pub struct Attributes {
    #[darling(default)]
    debug: bool,

    #[darling(default)]
    prefix: String,
}

impl Attributes {
    pub fn preferred_service_name(&self, impl_block: &ItemImpl) -> String {
        if self.prefix.is_empty() {
            let service_name = get_type_name(impl_block).expect("Must have valid service name");
            format!("{}_", pascal_to_snake_case(&service_name))
        } else {
            self.prefix.clone()
        }
    }
}

#[allow(clippy::match_wildcard_for_single_variants)]
pub fn ffi_service(attr: TokenStream, input: &TokenStream) -> TokenStream {
    let attributes = darling_parse!(Attributes, attr);
    let item = syn::parse2::<ItemImpl>(input.clone()).expect("Must be item.");
    let service_type = &item.self_ty;
    let mut function_descriptors = Vec::new();
    let mut rval = None;

    for impl_item in &item.items {
        if let ImplItem::Fn(method) = impl_item
            && let Visibility::Public(_) = &method.vis
            && let Some(descriptor) = generate_service_method(&attributes, &item, method)
        {
            if matches!(descriptor.method_type, MethodType::Constructor) {
                let rval_type = match &method.sig.output {
                    ReturnType::Type(_, b) => b.to_token_stream(),
                    _ => panic!("Must have return value"),
                };
                rval = Some(rval_type);
            }
            function_descriptors.push(descriptor);
        }
    }

    let rval = rval.expect("Service must have a constructor, a method without a `self` param, returning an `ffi::Result<Self, _>`).");
    let ffi_functions = function_descriptors.iter().map(|x| x.ffi_function_tokens.clone()).collect::<Vec<_>>();
    let ffi_dtor = generate_service_dtor(&attributes, &item);
    let ffi_method_ident = function_descriptors
        .iter()
        .filter(|x| matches!(x.method_type, MethodType::MethodSync(_) | MethodType::MethodAsync(_)))
        .map(|x| x.ident.clone())
        .collect::<Vec<_>>();
    let ffi_ctors = function_descriptors
        .iter()
        .filter(|x| matches!(x.method_type, MethodType::Constructor))
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

        impl <#lt> ::interoptopus::pattern::service::ServiceInfo for #service_type {
            type CtorResult = #rval;
        }

        impl <#lt> ::interoptopus::pattern::LibraryPatternInfo for #service_type {
            fn pattern_info() -> ::interoptopus::pattern::LibraryPattern {

                use ::interoptopus::lang::TypeInfo;
                use ::interoptopus::lang::FunctionInfo;

                let mut methods = ::std::vec::Vec::new();
                let mut ctors = ::std::vec::Vec::new();

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

                let service = ::interoptopus::pattern::service::ServiceDefinition::new(
                    ctors, dtor, methods,
                );

                service.assert_valid();

                ::interoptopus::pattern::LibraryPattern::Service(service)
            }
        }
    };

    if attributes.debug {
        prettyprint_tokenstream(&rval);
    }

    rval
}
