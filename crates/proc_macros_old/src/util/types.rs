use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::visit_mut::{VisitMut, visit_type_path_mut};
use syn::{GenericArgument, ItemImpl, PathArguments, Type, TypePath};

/// A type visitor that replaces all occurrences of `Self` in type paths with a
/// specified replacement type.
pub struct ReplaceSelf {
    replacement: TokenStream,
}

impl ReplaceSelf {
    pub fn new(replacement: TokenStream) -> Self {
        Self { replacement }
    }
}

impl VisitMut for ReplaceSelf {
    fn visit_type_path_mut(&mut self, path: &mut TypePath) {
        if path.qself.is_none() && path.path.segments.len() == 1 && path.path.segments[0].ident == "Self" {
            let ts = self.replacement.clone();
            let type_path: TypePath = syn::parse2(ts).unwrap();
            *path = type_path;
        } else {
            visit_type_path_mut(self, path);
        }
    }
}

/// Ugly, incomplete function to purge `'a` from a `Generic<'a, T>`.
///
/// TODO, should this be done by visitor as well?
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
