use darling::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Attribute, GenericArgument, PathArguments, Type};

pub fn extract_doc_lines(attributes: &[Attribute]) -> Vec<String> {
    let mut docs = Vec::new();

    for attr in attributes {
        // TODO: What is the un-ugly version of this?
        if &attr.path().to_token_stream().to_string() == "doc" {
            // TODO TODO
            // let list = NestedMeta::parse_meta_list(attribute.to_token_stream()).unwrap();
            // let rval = HashMap::<String, String>::from_nested_meta(list.first().unwrap()).ok().unwrap();
            // match attr.parse_meta().unwrap() {
            //     Meta::NameValue(x) => match x.lit {
            //         syn::Lit::Str(x) => {
            //             let the_line = x.value().to_string();
            //             docs.push(the_line);
            //         }
            //         _ => panic!("This was a bit unexpected."),
            //     },
            //     _ => panic!("This should not fail."),
            // }
        }
    }

    docs
}

/// Ugly, incomplete function to purge `'a` from a `Generic<'a, T>`.
pub fn purge_lifetimes_from_type(the_type: &Type) -> Type {
    let mut rval = the_type.clone();

    match &mut rval {
        Type::Path(x) => {
            for p in &mut x.path.segments {
                let mut still_has_parameter = false;

                match &mut p.arguments {
                    PathArguments::None => {}
                    PathArguments::AngleBracketed(angled_args) => {
                        let mut p = Punctuated::new();

                        for generic_arg in &mut angled_args.args {
                            match generic_arg {
                                GenericArgument::Lifetime(_) => {}
                                GenericArgument::Type(x) => {
                                    let x = purge_lifetimes_from_type(x);
                                    p.push(GenericArgument::Type(x));
                                }
                                GenericArgument::Constraint(x) => p.push(GenericArgument::Constraint(x.clone())),
                                GenericArgument::Const(x) => p.push(GenericArgument::Const(x.clone())),
                                _ => {}
                            }
                        }

                        still_has_parameter = !p.is_empty();
                        angled_args.args = p;
                    }
                    PathArguments::Parenthesized(_) => {}
                }

                if !still_has_parameter {
                    p.arguments = PathArguments::None;
                }
            }
        }
        Type::Reference(x) => {
            x.lifetime = None;
            x.elem = Box::new(purge_lifetimes_from_type(&x.elem))
        }
        Type::Ptr(x) => x.elem = Box::new(purge_lifetimes_from_type(&x.elem)),
        Type::Group(x) => x.elem = Box::new(purge_lifetimes_from_type(&x.elem)),
        _ => {}
    }

    rval
}
