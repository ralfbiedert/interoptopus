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
        .map(|attribute| {
            let list: HashMap<String, String> = FromMeta::from_meta(&attribute.meta)
                .expect(r#"Surrogates must be specified in the form of #[ffi_surrogates(field1 = "function1", field2 = "function2")]."#);
            (Some(attribute.span()), list)
        })
        .next()
        .unwrap_or_default()
}
