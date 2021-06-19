use crate::util::extract_doc_lines;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{AttributeArgs, Expr, GenericParam, ItemEnum, ItemStruct, ItemType, Lit, Type};

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
}

fn derive_variant_info(item: ItemEnum, idents: &[Ident], names: &[String], values: &[i32], docs: &[String]) -> TokenStream {
    let span = item.ident.span();
    let name = item.ident.to_string();
    let name_ident = syn::Ident::new(&name, span);

    quote! {
        impl interoptopus::lang::rust::VariantInfo for #name_ident {
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

pub fn ffi_type_enum(attr: FFITypeAttributes, input: TokenStream, item: ItemEnum) -> TokenStream {
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

    let variant_infos = derive_variant_info(item.clone(), &variant_idents, &variant_names, &variant_values, &variant_docs);
    let ctype_info_return = if attr.patterns.contains_key("success_enum") {
        quote! {
            let success_variant = Self::SUCCESS.variant_info();
            let the_success_enum = interoptopus::patterns::successenum::SuccessEnum::new(rval, success_variant);
            let the_pattern = interoptopus::patterns::TypePattern::SuccessEnum(the_success_enum);
            interoptopus::lang::c::CType::Pattern(the_pattern)
        }
    } else {
        quote! { interoptopus::lang::c::CType::Enum(rval) }
    };

    quote! {
        #input

        #variant_infos

        impl interoptopus::lang::rust::CTypeInfo for #name_ident {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::VariantInfo;

                let documentation = interoptopus::lang::c::Documentation::from_line(#doc_line);
                let mut rval = interoptopus::lang::c::EnumType::new(#name.to_string(), documentation);

                #({
                    rval.add_variant(Self::#variant_idents.variant_info());
                })*

                #ctype_info_return
            }
        }
    }
}

pub fn ffi_type_struct(attr: FFITypeAttributes, input: TokenStream, item: ItemStruct) -> TokenStream {
    let name = item.ident.to_string();
    let name_ident = syn::Ident::new(&name, item.ident.span());
    let c_struct_name = attr.name.unwrap_or(name);

    let mut generic_params = Vec::new();
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut field_docs = Vec::new();
    let mut generic_params_needing_bounds = Vec::new();

    let mut generics_params = quote! {};
    let mut where_clause = quote! {};

    let doc_line = extract_doc_lines(&item.attrs).join("\n");

    for generic in &item.generics.params {
        generic_params.push(quote! { #generic });

        match generic {
            GenericParam::Type(ty) => {
                let ident = ty.ident.clone();
                generic_params_needing_bounds.push(ident);
            }
            _ => {}
        }
    }

    if !generic_params.is_empty() {
        generics_params = quote! { < #(#generic_params),* > };

        if !generic_params_needing_bounds.is_empty() {
            if let Some(x) = item.generics.where_clause {
                // where_clause = quote! { #x, #( #generic_params_needing_bounds: interoptopus::lang::rust::CTypeInfo)*, }
                where_clause = quote! { #x }
            } else {
                // where_clause = quote! { where #( #generic_params_needing_bounds: interoptopus::lang::rust::CTypeInfo)*, }
            }
        }
    }

    for field in &item.fields {
        let name = field.ident.as_ref().expect("Field must be named").to_string();

        if attr.skip.contains_key(&name) {
            continue;
        }

        field_names.push(name.clone());
        field_docs.push(extract_doc_lines(&field.attrs).join("\n"));

        if attr.surrogates.contains_key(&name) {
            let lookup = attr.surrogates.get(&name).unwrap();
            let ident = syn::Ident::new(&lookup, item.ident.span());
            field_types.push(quote! { #ident()  })
        } else {
            let token = match &field.ty {
                Type::Path(x) => x.path.to_token_stream(),
                Type::Ptr(x) => x.to_token_stream(),
                Type::Reference(x) => x.to_token_stream(),
                _ => {
                    panic!("Unknown token: {:?}", field);
                }
            };

            field_types.push(quote! { < #token as interoptopus::lang::rust::CTypeInfo >::type_info()  })
        }
    }

    quote! {
        #input

        impl #generics_params interoptopus::lang::rust::CTypeInfo for #name_ident #generics_params #where_clause {

            fn type_info() -> interoptopus::lang::c::CType {

                let documentation = interoptopus::lang::c::Documentation::from_line(#doc_line);

                let mut rval = interoptopus::lang::c::CompositeType::with_documentation(#c_struct_name.to_string(), documentation);

                #({
                    let documentation = interoptopus::lang::c::Documentation::from_line(#field_docs);
                    let the_type = #field_types;
                    rval.add_field(interoptopus::lang::c::Field::with_documentation(#field_names.to_string(), the_type, documentation));
                })*

                interoptopus::lang::c::CType::Composite(rval)
            }
        }
    }
}

pub fn ffi_type_struct_opqaue(_attr: FFITypeAttributes, input: TokenStream, item: ItemStruct) -> TokenStream {
    let name = item.ident.to_string();
    let name_ident = syn::Ident::new(&name, item.ident.span());

    quote! {
        #input

        impl interoptopus::lang::rust::CTypeInfo for #name_ident {

            fn type_info() -> interoptopus::lang::c::CType {

                let mut rval = interoptopus::lang::c::OpaqueType::new(#name.to_string());

                interoptopus::lang::c::CType::Opaque(rval)
            }
        }
    }
}

pub fn ffi_type(attr: AttributeArgs, input: TokenStream) -> TokenStream {
    let ffi_attributes: FFITypeAttributes = FFITypeAttributes::from_list(&attr).unwrap();

    if let Ok(item) = syn::parse2::<ItemStruct>(input.clone()) {
        return if ffi_attributes.opaque {
            ffi_type_struct_opqaue(ffi_attributes, input, item)
        } else {
            ffi_type_struct(ffi_attributes, input, item)
        };
    }

    if let Ok(item) = syn::parse2::<ItemEnum>(input.clone()) {
        return ffi_type_enum(ffi_attributes, input, item);
    }

    if let Ok(_item) = syn::parse2::<ItemType>(input.clone()) {
        return input;
    }

    panic!("Annotation #[ffi_type] only works with structs and enum types.")
}
