use quote::ToTokens;
use syn::{Attribute, Expr, Lit, Meta};

/// From a let of attributes to an item, extracts the ones that are documentation, as strings.
pub fn extract_doc_lines(attributes: &[Attribute]) -> Vec<String> {
    let mut docs = Vec::new();

    for attr in attributes {
        if &attr.path().to_token_stream().to_string() != "doc" {
            continue;
        }

        if let Meta::NameValue(x) = &attr.meta {
            match &x.value {
                Expr::Lit(x) => match &x.lit {
                    Lit::Str(x) => {
                        docs.push(x.value());
                    }
                    _ => panic!("Unexpected content in doc string: not a string."),
                },
                _ => panic!("Unexpected content in doc string: not a literal."),
            }
        }
    }

    docs
}
