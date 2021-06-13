use darling::ToTokens;
use syn::{Attribute, Meta};

pub fn extract_doc_lines(attributes: &[Attribute]) -> Vec<String> {
    let mut docs = Vec::new();

    for attr in attributes {
        // TODO: What is the un-ugly version of this?
        if &attr.path.to_token_stream().to_string() == "doc" {
            match attr.parse_meta().unwrap() {
                Meta::NameValue(x) => match x.lit {
                    syn::Lit::Str(x) => {
                        let the_line = x.value().replacen(" ", "", 1).to_string();
                        docs.push(the_line);
                    }
                    _ => panic!("This was a bit unexpected."),
                },
                _ => panic!("This should not fail."),
            }
        }
    }

    docs
}
