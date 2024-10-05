use darling::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, GenericArgument, Lit, Meta, PathArguments, Type};

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
