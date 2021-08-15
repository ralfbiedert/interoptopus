use crate::types::enums::ffi_type_enum;
use crate::types::structs::ffi_type_struct;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{AttributeArgs, Field, ItemEnum, ItemStruct, ItemType, Visibility};

mod enums;
mod structs;

#[derive(Debug, FromMeta, Clone)]
pub struct Attributes {
    #[darling(default)]
    opaque: bool,

    #[darling(default)]
    patterns: HashMap<String, ()>,

    #[darling(default)]
    skip: HashMap<String, ()>,

    #[darling(default)]
    visibility: HashMap<String, String>,

    #[darling(default)]
    name: Option<String>,

    #[darling(default)]
    namespace: Option<String>,

    #[darling(default)]
    debug: bool,

    #[darling(default, rename = "unsafe")]
    unsfe: bool,
}

impl Attributes {
    pub fn assert_valid(&self) {
        if !self.skip.is_empty() && !self.unsfe {
            panic!("When using `surrogate` or `skip` you must also specify `unsafe`.")
        }
    }

    pub fn visibility_for_field(&self, field: &Field, name: &str) -> TokenStream {
        let mut rval = match &field.vis {
            Visibility::Public(_) => quote! { interoptopus::lang::c::Visibility::Public },
            _ => quote! { interoptopus::lang::c::Visibility::Private },
        };

        if let Some(x) = self.visibility.get(name) {
            rval = match x.as_str() {
                "public" => quote! { interoptopus::lang::c::Visibility::Public },
                "private" => quote! { interoptopus::lang::c::Visibility::Private },
                _ => panic!("Visibility must be `public` or `private`"),
            };
        }

        if let Some(x) = self.visibility.get("_") {
            rval = match x.as_str() {
                "public" => quote! { interoptopus::lang::c::Visibility::Public },
                "private" => quote! { interoptopus::lang::c::Visibility::Private },
                _ => panic!("Visibility must be `public` or `private`"),
            };
        }

        rval
    }
}

pub fn ffi_type(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let attributes: Attributes = Attributes::from_list(&attr).unwrap();

    let rval = if let Ok(item) = syn::parse2::<ItemStruct>(input.clone()) {
        ffi_type_struct(&attributes, input, item)
    } else if let Ok(item) = syn::parse2::<ItemEnum>(input.clone()) {
        ffi_type_enum(&attributes, input, item)
    } else if let Ok(_item) = syn::parse2::<ItemType>(input.clone()) {
        input
    } else {
        panic!("Annotation #[ffi_type] only works with structs and enum types.")
    };

    if attributes.debug {
        println!("{}", rval);
    }

    rval
}
