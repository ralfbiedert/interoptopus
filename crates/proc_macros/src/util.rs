use darling::ToTokens;
use proc_macro::TokenStream;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Attribute, Expr, File, GenericArgument, ItemImpl, Lit, Meta, PathArguments, Type, TypePath};

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
                    PathArguments::None | PathArguments::Parenthesized(_) => {}
                    PathArguments::AngleBracketed(angled_args) => {
                        let mut p = Punctuated::new();

                        for generic_arg in &mut angled_args.args {
                            match generic_arg {
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
                }

                if !still_has_parameter {
                    p.arguments = PathArguments::None;
                }
            }
        }
        Type::Reference(x) => {
            x.lifetime = None;
            x.elem = Box::new(purge_lifetimes_from_type(&x.elem));
        }
        Type::Ptr(x) => x.elem = Box::new(purge_lifetimes_from_type(&x.elem)),
        Type::Group(x) => x.elem = Box::new(purge_lifetimes_from_type(&x.elem)),
        _ => {}
    }

    rval
}

pub fn get_type_name(impl_block: &ItemImpl) -> Option<String> {
    // Dereference the Box<Type> to get &Type
    let ty = impl_block.self_ty.as_ref();

    match ty {
        Type::Path(TypePath { path, .. }) => {
            // Extract the type name from the path segments
            let type_name = path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>().join("::");
            Some(type_name)
        }
        _ => None, // Handle other types accordingly
    }
}

pub fn pascal_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                let prev = chars[i - 1];
                let next = if i + 1 < len { chars[i + 1] } else { '\0' };
                if prev.is_lowercase() || (prev.is_uppercase() && next.is_lowercase()) {
                    result.push('_');
                }
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

pub fn prettyprint_tokenstream(tokens: &TokenStream2) -> TokenStream {
    let rval1: proc_macro::TokenStream = tokens.clone().into();
    let syntax_tree: File = parse_macro_input!(rval1 as File);
    let string = prettyplease::unparse(&syntax_tree);
    println!("---------------------------------------------------------------------------------------------------");
    println!("{string}");
    syntax_tree.into_token_stream().into()
}
