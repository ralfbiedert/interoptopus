use darling::FromMeta;
use darling::ToTokens;
use proc_macro2::Span;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::Attribute;

pub fn read_surrogates(attributes: &[Attribute]) -> (Option<Span>, HashMap<String, String>) {
    attributes
        .iter()
        .filter(|x| x.to_token_stream().to_string().contains("ffi_surrogate"))
        .filter_map(|attribute| {
            // let list = NestedMeta::parse_meta_list(attribute.to_token_stream()).unwrap();
            let list: HashMap<String, String> = FromMeta::from_meta(&attribute.meta).unwrap();
            Some((Some(attribute.span()), list))
        })
        .next()
        .unwrap_or_default()
}
