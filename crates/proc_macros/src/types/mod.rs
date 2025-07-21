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
#[allow(clippy::zero_sized_map_values)]
#[allow(clippy::struct_excessive_bools)]
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

    // Disabled for now
    // #[darling(default)]
    // align: Option<usize>,
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

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub enum TypeRepresentation {
    C,
    Transparent,
    Packed,
    Opaque,
    Primitive(&'static str),
}

impl Attributes {
    #[allow(clippy::useless_let_if_seq)]
    pub fn visibility_for_field(&self, field: &Field, name: &str) -> TokenStream {
        let mut rval = if let Visibility::Public(_) = &field.vis {
            quote! { interoptopus::lang::Visibility::Public }
        } else {
            quote! { interoptopus::lang::Visibility::Private }
        };

        if let Some(x) = self.visibility.get(name) {
            rval = match x.as_str() {
                "public" => quote! { interoptopus::lang::Visibility::Public },
                "private" => quote! { interoptopus::lang::Visibility::Private },
                _ => panic!("Visibility must be `public` or `private`"),
            };
        }

        if let Some(x) = self.visibility.get("_all") {
            rval = match x.as_str() {
                "public" => quote! { interoptopus::lang::Visibility::Public },
                "private" => quote! { interoptopus::lang::Visibility::Private },
                _ => panic!("Visibility must be `public` or `private`"),
            };
        }

        rval
    }

    #[rustfmt::skip]
    const fn type_repr_align(&self) -> (TypeRepresentation, Option<usize>) {
        let mut rval = (TypeRepresentation::C, None);

        if self.opaque { rval.0 = TypeRepresentation::Opaque; }
        if self.transparent { rval.0 = TypeRepresentation::Transparent; }
        if self.packed { rval.0 = TypeRepresentation::Packed; }
        if self.u8 { rval.0 = TypeRepresentation::Primitive("u8"); }
        if self.u16 { rval.0 = TypeRepresentation::Primitive("u16"); }
        if self.u32 { rval.0 = TypeRepresentation::Primitive("u32"); }
        if self.u64 { rval.0 = TypeRepresentation::Primitive("u64"); }

        rval
    }
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
        println!("{rval}");
    }

    rval
}
