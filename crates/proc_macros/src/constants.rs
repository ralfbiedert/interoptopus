use crate::util::extract_doc_lines;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemConst;

#[derive(Debug, FromMeta)]
pub struct Attributes {}

pub fn ffi_constant(_attr: TokenStream, input: &TokenStream) -> TokenStream {
    let const_item: ItemConst = syn::parse2(input.clone()).expect("Must be item.");

    let const_ident = const_item.ident;
    let const_name = const_ident.to_string();
    let doc_line = extract_doc_lines(&const_item.attrs).join("\n");

    quote! {
        #input

        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct #const_ident {}

        unsafe impl ::interoptopus::lang::ConstantInfo for #const_ident {
            fn constant_info() -> interoptopus::lang::Constant {

                let documentation = ::interoptopus::lang::Documentation::from_line(#doc_line);
                let meta = ::interoptopus::lang::Meta::with_documentation(documentation);
                let value = ::interoptopus::lang::ConstantValue::from(#const_ident);

                ::interoptopus::lang::Constant::new(#const_name.to_string(), value, meta)
            }
        }
    }
}
