use crate::types::{Attributes, TypeRepresentation};
use crate::util::extract_doc_lines;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use std::iter::zip;
use syn::{Expr, ItemEnum, Lit};

pub enum VariantKind {
    Unit(usize),
    Typed(usize, Ident),
}

fn derive_variant_info(item: &ItemEnum, idents: &[Ident], names: &[String], values: &[VariantKind], docs: &[String]) -> TokenStream {
    let name = item.ident.to_string();
    let name_ident = syn::Ident::new(&name, item.ident.span());

    let entries = zip(idents, values)
        .map(|(id, x)| {
            let span = id.span();
            match x {
                VariantKind::Unit(v) => quote_spanned! (span => ::interoptopus::lang::VariantKind::Unit(#v) ),
                // TODO
                VariantKind::Typed(i, v) => quote_spanned! (span => ::interoptopus::lang::VariantKind::Unit(#i) ),
            }
        })
        .collect::<Vec<_>>();

    quote! {
        unsafe impl ::interoptopus::lang::VariantInfo for #name_ident {
            fn variant_info(&self) -> ::interoptopus::lang::Variant {
                match self {
                    #(
                       Self::#idents => {
                            let documentation = ::interoptopus::lang::Documentation::from_line(#docs);
                            ::interoptopus::lang::Variant::new(#names.to_string(), #entries, documentation)
                       },
                    )*
                }
            }
        }
    }
}

pub fn ffi_type_enum(attributes: &Attributes, _input: TokenStream, mut item: ItemEnum) -> TokenStream {
    let doc_line = extract_doc_lines(&item.attrs).join("\n");
    let (type_repr, align) = attributes.type_repr_align();

    let span = item.ident.span();
    let name = item.ident.to_string();
    let ffi_name = attributes.name.clone().unwrap_or_else(|| name.clone());
    let name_ident = syn::Ident::new(&name, span);
    let namespace = attributes.namespace.clone().unwrap_or_default();

    let mut variant_names = Vec::new();
    let mut variant_idents = Vec::new();
    let mut variant_kinds = Vec::new();
    let mut variant_docs = Vec::new();
    let mut next_id = 0;

    for variant in &item.variants {
        let ident = variant.ident.to_string();
        let variant_doc_line = extract_doc_lines(&variant.attrs).join("\n");

        let this_id = if let Some((_, e)) = &variant.discriminant {
            match e {
                Expr::Lit(e) => match &e.lit {
                    Lit::Int(x) => {
                        let number = x.base10_parse().expect("Must be number");
                        next_id = number + 1;
                        VariantKind::Unit(number)
                    }
                    _ => panic!("Unknown token."),
                },
                _ => panic!("Unknown token."),
            }
        } else {
            let id = next_id;
            next_id += 1;
            VariantKind::Unit(id)
        };

        if !attributes.skip.contains_key(&ident) {
            variant_idents.push(syn::Ident::new(&ident, span));
            variant_names.push(ident);
            variant_kinds.push(this_id);
            variant_docs.push(variant_doc_line);
        }
    }

    let variant_infos = derive_variant_info(&item, &variant_idents, &variant_names, &variant_kinds, &variant_docs);

    let ctype_info_return = if attributes.error {
        quote! {
            use ::interoptopus::pattern::result::FFIError as _;
            let success_variant = Self::SUCCESS.variant_info();
            let panic_variant = Self::PANIC.variant_info();
            let the_success_enum = ::interoptopus::pattern::result::FFIErrorEnum::new(rval, success_variant, panic_variant);
            let the_pattern = ::interoptopus::pattern::TypePattern::FFIErrorEnum(the_success_enum);
            ::interoptopus::lang::Type::Pattern(the_pattern)
        }
    } else {
        quote! { ::interoptopus::lang::Type::Enum(rval) }
    };

    let attr_align = align.map_or_else(
        || quote! {},
        |x| {
            let x_lit = syn::LitInt::new(&x.to_string(), Span::call_site());
            quote! { , align( #x_lit ) }
        },
    );

    let align = align.map_or_else(|| quote! { None }, |x| quote! { Some(#x) });

    let layout = match type_repr {
        TypeRepresentation::C => quote! { ::interoptopus::lang::Layout::C },
        TypeRepresentation::Transparent => quote! { ::interoptopus::lang::Layout::Transparent },
        TypeRepresentation::Primitive(_) => quote! { compile_error!("TODO") },
        _ => quote! { compile_error!("Unsupported repr for enum") },
    };

    let attr_repr = match type_repr {
        TypeRepresentation::C => quote! { #[repr(C #attr_align)] },
        TypeRepresentation::Transparent => quote! { #[repr(transparent #attr_align)] },
        TypeRepresentation::Primitive(x) => quote! { #[repr(#x #attr_align)] },
        _ => quote! { compile_error!("Unsupported repr for enum") },
    };

    if item.attrs.iter().any(|attr| attr.path().is_ident("repr")) {
        panic!("Since 0.15 you must not add any `#[repr()] attributes to your enum; Interoptopus will handle that for you.");
    } else {
        item.attrs.push(syn::parse_quote!(#attr_repr));
    }

    quote! {
        #item

        #variant_infos

        unsafe impl ::interoptopus::lang::TypeInfo for #name_ident {
            fn type_info() -> ::interoptopus::lang::Type {
                use ::interoptopus::lang::VariantInfo;

                let mut variants = ::std::vec::Vec::new();
                let documentation = ::interoptopus::lang::Documentation::from_line(#doc_line);
                let mut meta = ::interoptopus::lang::Meta::with_namespace_documentation(#namespace.to_string(), documentation);

                #({
                    variants.push(Self::#variant_idents.variant_info());
                })*

                let repr = ::interoptopus::lang::Representation::new(#layout, #align);
                let rval = ::interoptopus::lang::Enum::new(#ffi_name.to_string(), variants, meta, repr);

                #ctype_info_return
            }
        }
    }
}
