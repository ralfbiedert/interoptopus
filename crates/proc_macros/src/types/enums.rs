use crate::types::{Attributes, TypeRepresentation};
use crate::util::extract_doc_lines;
use proc_macro2::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::{ToTokens, quote, quote_spanned};
use syn::{Expr, Fields, ItemEnum, Lit};

pub enum VariantKind {
    Unit(usize),
    Typed(usize, TokenStream),
}

#[allow(clippy::too_many_lines)]
pub fn ffi_type_enum(attributes: &Attributes, _input: TokenStream, mut item: ItemEnum) -> TokenStream {
    let doc_line = extract_doc_lines(&item.attrs).join("\n");
    let (type_repr, _) = attributes.type_repr_align();

    let span = item.ident.span();
    let name = item.ident.to_string();
    let ffi_name = attributes.name.clone().unwrap_or_else(|| name.clone());
    let name_ident = syn::Ident::new(&name, span);
    let namespace = attributes.namespace.clone().unwrap_or_default();
    let mut variants = Vec::new();

    let mut next_id = 0;

    for variant in &item.variants {
        let ident = variant.ident.to_string();
        let variant_doc_line = extract_doc_lines(&variant.attrs).join("\n");

        let discriminant = if let Some((_, e)) = &variant.discriminant {
            match e {
                Expr::Lit(e) => match &e.lit {
                    Lit::Int(x) => {
                        let number = x.base10_parse().expect("Must be number");
                        next_id = number + 1;
                        number
                    }
                    _ => panic!("Unknown token."),
                },
                _ => panic!("Unknown token."),
            }
        } else {
            let id = next_id;
            next_id += 1;
            id
        };

        let variant_kind = match &variant.fields {
            Fields::Unit => VariantKind::Unit(discriminant),
            Fields::Unnamed(x) => {
                let field_ty = x.unnamed.next().expect("Must have one unnamed field");
                VariantKind::Typed(discriminant, field_ty.to_token_stream())
            }
            Fields::Named(_) => panic!("Named variants are not supported."),
        };

        match variant_kind {
            VariantKind::Unit(x) => {
                let tokens = quote_spanned!(variant.ident.span() => {
                    let documentation = ::interoptopus::lang::Documentation::from_line(#variant_doc_line);
                    let kind = ::interoptopus::lang::VariantKind::Unit(#x);
                    let variant = ::interoptopus::lang::Variant::new(#ident.to_string(), kind, documentation);
                    variants.push(variant);
                });
                variants.push(tokens);
            }
            VariantKind::Typed(x, ts) => {
                let tokens = quote_spanned!(variant.ident.span() => {
                    let documentation = ::interoptopus::lang::Documentation::from_line(#variant_doc_line);
                    let ty = ::std::boxed::Box::new(<#ts as ::interoptopus::lang::TypeInfo>::type_info());
                    let kind = ::interoptopus::lang::VariantKind::Typed(#x, ty);
                    let variant = ::interoptopus::lang::Variant::new(#ident.to_string(), kind, documentation);
                    variants.push(variant);
                });
                variants.push(tokens);
            }
        }
    }

    let layout = match type_repr {
        TypeRepresentation::C => quote! { ::interoptopus::lang::Layout::C },
        TypeRepresentation::Transparent => quote! { ::interoptopus::lang::Layout::Transparent },
        TypeRepresentation::Primitive(_) => quote! { compile_error!("TODO") },
        _ => quote! { compile_error!("Unsupported repr for enum") },
    };

    let attr_align = quote! { #[repr(u32) ]};

    if item.attrs.iter().any(|attr| attr.path().is_ident("repr")) {
        panic!("Since 0.15 you must not add any `#[repr()] attributes to your enum; Interoptopus will handle that for you.");
    } else {
        item.attrs.push(syn::parse_quote!(#attr_align));
    }

    quote! {
        #item

        unsafe impl ::interoptopus::lang::TypeInfo for #name_ident {
            fn type_info() -> ::interoptopus::lang::Type {
                let mut variants = ::std::vec::Vec::new();
                let documentation = ::interoptopus::lang::Documentation::from_line(#doc_line);
                let mut meta = ::interoptopus::lang::Meta::with_namespace_documentation(#namespace.to_string(), documentation);

                #({
                    #variants
                })*

                let repr = ::interoptopus::lang::Representation::new(#layout, None);
                let rval = ::interoptopus::lang::Enum::new(#ffi_name.to_string(), variants, meta, repr);
                ::interoptopus::lang::Type::Enum(rval)
            }
        }
    }
}
