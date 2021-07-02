use crate::util::extract_doc_lines;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{AttributeArgs, Expr, GenericParam, ItemEnum, ItemStruct, ItemType, Lit, Type, Visibility};

#[derive(Debug, FromMeta, Clone)]
pub struct FFITypeAttributes {
    #[darling(default)]
    opaque: bool,

    #[darling(default)]
    surrogates: HashMap<String, String>,

    #[darling(default)]
    patterns: HashMap<String, ()>,

    #[darling(default)]
    skip: HashMap<String, ()>,

    #[darling(default)]
    name: Option<String>,

    #[darling(default)]
    namespace: Option<String>,

    #[darling(default)]
    debug: bool,
}

fn derive_variant_info(item: ItemEnum, idents: &[Ident], names: &[String], values: &[i32], docs: &[String]) -> TokenStream {
    let span = item.ident.span();
    let name = item.ident.to_string();
    let name_ident = syn::Ident::new(&name, span);

    quote! {
        unsafe impl interoptopus::lang::rust::VariantInfo for #name_ident {
            fn variant_info(&self) -> interoptopus::lang::c::Variant {
                match self {
                    #(
                       Self::#idents => {
                            let documentation = interoptopus::lang::c::Documentation::from_line(#docs);
                            interoptopus::lang::c::Variant::new(#names.to_string(), #values as usize, documentation)
                       },
                    )*
                }
            }
        }
    }
}

pub fn ffi_type_enum(attr: &FFITypeAttributes, input: TokenStream, item: ItemEnum) -> TokenStream {
    let span = item.ident.span();
    let name = item.ident.to_string();
    let name_ident = syn::Ident::new(&name, span);

    let mut variant_names = Vec::new();
    let mut variant_idents = Vec::new();
    let mut variant_values = Vec::new();
    let mut variant_docs = Vec::new();

    let mut next_id = 0;

    let doc_line = extract_doc_lines(&item.attrs).join("\n");

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

        if !attr.skip.contains_key(&ident) {
            variant_idents.push(syn::Ident::new(&ident, span));
            variant_names.push(ident);
            variant_values.push(this_id);
            variant_docs.push(variant_doc_line);
        }
    }

    let variant_infos = derive_variant_info(item, &variant_idents, &variant_names, &variant_values, &variant_docs);

    let ctype_info_return = if attr.patterns.contains_key("success_enum") {
        quote! {
            let success_variant = Self::SUCCESS.variant_info();
            let the_success_enum = interoptopus::patterns::success_enum::SuccessEnum::new(rval, success_variant);
            let the_pattern = interoptopus::patterns::TypePattern::SuccessEnum(the_success_enum);
            interoptopus::lang::c::CType::Pattern(the_pattern)
        }
    } else {
        quote! { interoptopus::lang::c::CType::Enum(rval) }
    };

    quote! {
        #input

        #variant_infos

        unsafe impl interoptopus::lang::rust::CTypeInfo for #name_ident {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::VariantInfo;

                let mut variants = std::vec::Vec::new();
                let documentation = interoptopus::lang::c::Documentation::from_line(#doc_line);
                let meta = interoptopus::lang::c::Meta::with_documentation(documentation);

                #({
                    variants.push(Self::#variant_idents.variant_info());
                })*

                let rval = interoptopus::lang::c::EnumType::new(#name.to_string(), variants, meta);

                #ctype_info_return
            }
        }
    }
}

pub fn ffi_type_struct(attr: &FFITypeAttributes, input: TokenStream, item: ItemStruct) -> TokenStream {
    let name = item.ident.to_string();
    let name_ident = syn::Ident::new(&name, item.ident.span());
    let c_struct_name = attr.name.clone().unwrap_or(name);
    let namespace = attr.namespace.clone().unwrap_or_else(|| "".to_string());

    let mut generic_params = Vec::new();
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut field_docs = Vec::new();
    let mut field_visibility = Vec::new();
    let mut generic_params_needing_bounds = Vec::new();

    let mut generics_params = quote! {};
    let mut where_clause = quote! {};

    let doc_line = extract_doc_lines(&item.attrs).join("\n");

    for generic in &item.generics.params {
        generic_params.push(quote! { #generic });

        if let GenericParam::Type(ty) = generic {
            let ident = ty.ident.clone();
            generic_params_needing_bounds.push(ident);
        }
    }

    if !generic_params.is_empty() {
        generics_params = quote! { < #(#generic_params),* > };

        if !generic_params_needing_bounds.is_empty() {
            if let Some(where_cl) = item.generics.where_clause {
                where_clause = quote! { #where_cl }
            }
        }
    }

    for (i, field) in item.fields.iter().enumerate() {
        let name = field.ident.as_ref().map(|x| x.to_string()).unwrap_or_else(|| format!("x{}", i.to_string()));

        if attr.skip.contains_key(&name) {
            continue;
        }

        let visibility = match &field.vis {
            Visibility::Public(_) => quote! { interoptopus::lang::c::Visibility::Public },
            _ => quote! { interoptopus::lang::c::Visibility::Private },
        };

        field_names.push(name.clone());
        field_docs.push(extract_doc_lines(&field.attrs).join("\n"));
        field_visibility.push(visibility);

        if attr.surrogates.contains_key(&name) {
            let lookup = attr.surrogates.get(&name).unwrap();
            let ident = syn::Ident::new(&lookup, item.ident.span());
            field_types.push(quote! { #ident()  })
        } else {
            let token = match &field.ty {
                Type::Path(x) => {
                    if let Some(qself) = &x.qself {
                        let ty = &qself.ty;

                        // Ok, I really don't know if this is kosher. After some debugging it seems to work,
                        // but this is probably brittle AF.

                        let first = x.path.segments.first().expect("Must have last path segment.");
                        let middle = x.path.segments.iter().skip(1).rev().skip(1).rev().collect::<Vec<_>>();
                        let last = x.path.segments.last().expect("Must have last path segment.");
                        quote! { < #ty as #first #(:: #middle)*> :: #last }
                    } else {
                        x.path.to_token_stream()
                    }
                }
                Type::Ptr(x) => x.to_token_stream(),
                Type::Reference(x) => x.to_token_stream(),
                _ => {
                    panic!("Unknown token: {:?}", field);
                }
            };

            field_types.push(quote! { < #token as interoptopus::lang::rust::CTypeInfo >::type_info()  })
        }
    }

    let rval_builder = if attr.opaque {
        quote! {
            let mut rval = interoptopus::lang::c::OpaqueType::new(name, meta);
            interoptopus::lang::c::CType::Opaque(rval)
        }
    } else {
        quote! {
            let rval = interoptopus::lang::c::CompositeType::with_meta(name, fields, meta);
            interoptopus::lang::c::CType::Composite(rval)
        }
    };

    let fields = if attr.opaque {
        quote! {}
    } else {
        quote! {
            #({
                let documentation = interoptopus::lang::c::Documentation::from_line(#field_docs);
                let the_type = #field_types;
                let field = interoptopus::lang::c::Field::with_documentation(#field_names.to_string(), the_type, #field_visibility, documentation);
                fields.push(field);
            })*
        }
    };

    let name = if let Some(name) = &attr.name {
        quote! {
            let name = #name.to_string();
        }
    } else {
        quote! {
            #({
                generics.push(<#generic_params_needing_bounds as interoptopus::lang::rust::CTypeInfo>::type_info().name_within_lib());
            })*

            let name = format!("{}{}", #c_struct_name.to_string(), generics.join(""));
        }
    };

    quote! {
        #input

        unsafe impl #generics_params interoptopus::lang::rust::CTypeInfo for #name_ident #generics_params #where_clause {

            fn type_info() -> interoptopus::lang::c::CType {
                let documentation = interoptopus::lang::c::Documentation::from_line(#doc_line);
                let mut meta = interoptopus::lang::c::Meta::with_namespace_documentation(#namespace.to_string(), documentation);
                let mut fields: std::vec::Vec<interoptopus::lang::c::Field> = std::vec::Vec::new();
                let mut generics: std::vec::Vec<String> = std::vec::Vec::new();

                #name

                #fields

                #rval_builder
            }
        }
    }
}

pub fn ffi_type(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let ffi_attributes: FFITypeAttributes = FFITypeAttributes::from_list(&attr).unwrap();

    let rval = if let Ok(item) = syn::parse2::<ItemStruct>(input.clone()) {
        ffi_type_struct(&ffi_attributes, input, item)
    } else if let Ok(item) = syn::parse2::<ItemEnum>(input.clone()) {
        ffi_type_enum(&ffi_attributes, input, item)
    } else if let Ok(_item) = syn::parse2::<ItemType>(input.clone()) {
        input
    } else {
        panic!("Annotation #[ffi_type] only works with structs and enum types.")
    };

    if ffi_attributes.debug {
        println!("{}", rval);
    }

    rval
}
