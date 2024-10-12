use crate::macros::darling_parse;
use crate::types::enums::ffi_type_enum;
use crate::types::structs::ffi_type_struct;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Field, ItemEnum, ItemStruct, ItemType, Visibility};

mod enums;
mod structs;

#[derive(Debug, FromMeta, Clone)]
pub struct Attributes {
    #[darling(default)]
    opaque: bool,

    #[darling(default)]
    transparent: bool,

    #[darling(default)]
    packed: bool,

    #[darling(default)]
    u8: bool,

    #[darling(default)]
    u16: bool,

    #[darling(default)]
    u32: bool,

    #[darling(default)]
    u64: bool,

    #[darling(default)]
    align: Option<usize>,

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
}

impl Attributes {
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

        if let Some(x) = self.visibility.get("_all") {
            rval = match x.as_str() {
                "public" => quote! { interoptopus::lang::c::Visibility::Public },
                "private" => quote! { interoptopus::lang::c::Visibility::Private },
                _ => panic!("Visibility must be `public` or `private`"),
            };
        }

        rval
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeRepresentation {
    C,
    Transparent,
    Packed,
    Opaque,
    Primitive(&'static str),
}

#[rustfmt::skip]
fn type_repr_align(attributes: &Attributes) -> (TypeRepresentation, Option<usize>) {
    let mut rval = (TypeRepresentation::C, attributes.align);

    if attributes.opaque { rval.0 = TypeRepresentation::Opaque; }
    if attributes.transparent { rval.0 = TypeRepresentation::Transparent; }
    if attributes.packed { rval.0 = TypeRepresentation::Packed; }
    if attributes.u8 { rval.0 = TypeRepresentation::Primitive("u8"); }
    if attributes.u16 { rval.0 = TypeRepresentation::Primitive("u16"); }
    if attributes.u32 { rval.0 = TypeRepresentation::Primitive("u32"); }
    if attributes.u64 { rval.0 = TypeRepresentation::Primitive("u64"); }

    rval
}

pub fn ffi_type(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = darling_parse!(Attributes, attr);

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
