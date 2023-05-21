use crate::types::Attributes;
use crate::util::extract_doc_lines;
use darling::ToTokens;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Expr, ItemEnum, Lit};

fn assert_valid_repr(_attributes: &Attributes, item: &ItemEnum) {
    if !item.attrs.iter().any(|x| x.to_token_stream().to_string().contains("repr")) {
        panic!("Enum `{}` must have `#[repr()]` annotation.", item.ident);
    }
}

fn derive_variant_info(item: ItemEnum, idents: &[Ident], names: &[String], values: &[i32], docs: &[String]) -> TokenStream {
    let name = item.ident.to_string();
    let name_ident = syn::Ident::new(&name, item.ident.span());

    quote! {
        unsafe impl ::interoptopus::lang::rust::VariantInfo for #name_ident {
            fn variant_info(&self) -> ::interoptopus::lang::c::Variant {
                match self {
                    #(
                       Self::#idents => {
                            let documentation = ::interoptopus::lang::c::Documentation::from_line(#docs);
                            ::interoptopus::lang::c::Variant::new(#names.to_string(), #values as usize, documentation)
                       },
                    )*
                }
            }
        }
    }
}

pub fn ffi_type_enum(attributes: &Attributes, input: TokenStream, item: ItemEnum) -> TokenStream {
    let doc_line = extract_doc_lines(&item.attrs).join("\n");

    assert_valid_repr(attributes, &item);

    let span = item.ident.span();
    let name = item.ident.to_string();
    let ffi_name = attributes.name.clone().unwrap_or_else(|| name.clone());
    let name_ident = syn::Ident::new(&name, span);
    let namespace = attributes.namespace.clone().unwrap_or_default();

    let mut variant_names = Vec::new();
    let mut variant_idents = Vec::new();
    let mut variant_values = Vec::new();
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

        if !attributes.skip.contains_key(&ident) {
            variant_idents.push(syn::Ident::new(&ident, span));
            variant_names.push(ident);
            variant_values.push(this_id);
            variant_docs.push(variant_doc_line);
        }
    }

    let variant_infos = derive_variant_info(item, &variant_idents, &variant_names, &variant_values, &variant_docs);

    let ctype_info_return = if attributes.patterns.contains_key("ffi_error") {
        quote! {
            use ::interoptopus::patterns::result::FFIError as _;
            let success_variant = Self::SUCCESS.variant_info();
            let the_success_enum = ::interoptopus::patterns::result::FFIErrorEnum::new(rval, success_variant);
            let the_pattern = ::interoptopus::patterns::TypePattern::FFIErrorEnum(the_success_enum);
            ::interoptopus::lang::c::CType::Pattern(the_pattern)
        }
    } else {
        quote! { ::interoptopus::lang::c::CType::Enum(rval) }
    };

    quote! {
        #input

        #variant_infos

        unsafe impl ::interoptopus::lang::rust::CTypeInfo for #name_ident {
            fn type_info() -> ::interoptopus::lang::c::CType {
                use ::interoptopus::lang::rust::VariantInfo;

                let mut variants = ::std::vec::Vec::new();
                let documentation = ::interoptopus::lang::c::Documentation::from_line(#doc_line);
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(#namespace.to_string(), documentation, None);

                #({
                    variants.push(Self::#variant_idents.variant_info());
                })*

                let rval = ::interoptopus::lang::c::EnumType::new(#ffi_name.to_string(), variants, meta);

                #ctype_info_return
            }
        }
    }
}
