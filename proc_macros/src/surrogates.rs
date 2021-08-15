use darling::FromMeta;
use darling::ToTokens;
use proc_macro2::Span;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::Attribute;

// #[derive(Debug, Default, FromMeta)]
// pub struct AttributeSurrogate(#[darling(default)] HashMap<String, String>);

pub fn read_surrogates(attributes: &[Attribute]) -> (Option<Span>, HashMap<String, String>) {
    attributes
        .iter()
        .filter(|x| x.to_token_stream().to_string().contains("ffi_surrogate"))
        .filter_map(|attribute| {
            let meta = attribute.parse_meta().ok()?;
            let rval = HashMap::<String, String>::from_meta(&meta).ok()?;
            Some((Some(attribute.span()), rval))
        })
        .next()
        .unwrap_or_default()
}
