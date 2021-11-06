use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemConst};

use crate::util::extract_doc_lines;

#[derive(Debug, FromMeta)]
pub struct Attributes {}

pub fn ffi_constant(_attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let const_item: ItemConst = syn::parse2(input.clone()).expect("Must be item.");

    let const_ident = const_item.ident;
    let const_name = const_ident.to_string();

    let doc_line = extract_doc_lines(&const_item.attrs).join("\n");

    quote! {
        #input

        #[allow(non_camel_case_types)]
        pub(crate) struct #const_ident {}

        unsafe impl ::interoptopus::lang::rust::ConstantInfo for #const_ident {
            fn constant_info() -> interoptopus::lang::c::Constant {

                let documentation = ::interoptopus::lang::c::Documentation::from_line(#doc_line);
                let meta = ::interoptopus::lang::c::Meta::with_documentation(documentation);
                let value = ::interoptopus::lang::c::ConstantValue::from(#const_ident);

                ::interoptopus::lang::c::Constant::new(#const_name.to_string(), value, meta)
            }
        }
    }
}
