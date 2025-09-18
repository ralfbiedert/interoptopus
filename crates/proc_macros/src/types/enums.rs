use crate::types::{Attributes, TypeRepresentation};
use crate::util::extract_doc_lines;
use proc_macro2::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::{ToTokens, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Expr, Fields, GenericParam, ItemEnum, Lit};

pub enum VariantKind {
    Unit(usize),
    Typed(usize, TokenStream),
}

#[allow(clippy::struct_field_names)]
struct GenericInfo {
    param_param: TokenStream,
    param_struct: TokenStream,
    param_where: TokenStream,
}

struct VariantProcessResult {
    variants: Vec<TokenStream>,
    ser_arms: Vec<TokenStream>,
    de_arms: Vec<TokenStream>,
    storage_arms: Vec<TokenStream>,
}

/// Process generic parameters and return structured information
fn process_generic_parameters(item: &ItemEnum) -> GenericInfo {
    let mut has_generics = false;
    let mut generic_parameter_tokens = Vec::new();
    let mut generic_struct_tokens = Vec::new();
    let mut generic_where_tokens = Vec::new();

    for generic in &item.generics.params {
        match generic {
            GenericParam::Lifetime(lt) => {
                let ident = lt.lifetime.ident.clone();
                let lt = syn::Lifetime::new(&format!("'{ident}"), item.span());
                generic_parameter_tokens.push(quote! { #lt });
                generic_struct_tokens.push(quote! { #lt });
            }
            GenericParam::Type(ty) => {
                let ident = ty.ident.clone();
                let whre = ty.bounds.to_token_stream();
                generic_parameter_tokens.push(quote! { #ident });
                generic_struct_tokens.push(quote! { #ident });
                generic_where_tokens.push(quote! { #ident: interoptopus::lang::TypeInfo });
                if !whre.to_string().is_empty() {
                    generic_where_tokens.push(quote! { #ident: #whre });
                }
            }
            GenericParam::Const(x) => {
                let ident = x.ident.clone();
                let ty = x.ty.to_token_stream();
                generic_parameter_tokens.push(quote! { const #ident: #ty });
                generic_struct_tokens.push(quote! { #ident });
            }
        }

        has_generics = true;
    }

    let (param_param, param_struct, param_where) = if has_generics {
        (quote! { < #(#generic_parameter_tokens),* > }, quote! { < #(#generic_struct_tokens),* > }, quote! { where #(#generic_where_tokens),*  })
    } else {
        (quote! {}, quote! {}, quote! {})
    };

    GenericInfo { param_param, param_struct, param_where }
}

/// Process a single variant and return its discriminant value
fn process_variant_discriminant(variant: &syn::Variant, next_id: &mut usize) -> usize {
    if let Some((_, e)) = &variant.discriminant {
        match e {
            Expr::Lit(e) => match &e.lit {
                Lit::Int(x) => {
                    let number = x.base10_parse().expect("Must be number");
                    *next_id = number + 1;
                    number
                }
                _ => panic!("Unknown token."),
            },
            _ => panic!("Unknown token."),
        }
    } else {
        let id = *next_id;
        *next_id += 1;
        id
    }
}

/// Process unit variant and generate tokens
fn process_unit_variant(
    variant: &syn::Variant,
    discriminant: usize,
    name_ident: &syn::Ident,
    attributes: &Attributes,
) -> (TokenStream, Option<TokenStream>, Option<TokenStream>, Option<TokenStream>) {
    let ident = variant.ident.to_string();
    let ident_tok = &variant.ident;
    let variant_doc_line = extract_doc_lines(&variant.attrs).join("\n");

    let variant_token = quote_spanned!(variant.ident.span() => {
        let docs = ::interoptopus::lang::Docs::from_line(#variant_doc_line);
        let kind = ::interoptopus::lang::VariantKind::Unit(#discriminant);
        let variant = ::interoptopus::lang::Variant::new(#ident.to_string(), kind, docs);
        variants.push(variant);
    });

    let (ser_arm, de_arm, storage_arm) = if attributes.wired {
        (
            Some(quote! {
                #name_ident::#ident_tok => {
                    #discriminant.ser(output)?;
                }
            }),
            Some(quote! {
                #discriminant => Ok(#name_ident::#ident_tok),
            }),
            Some(quote! {
                #name_ident::#ident_tok => 0usize.storage_size(), // size of discriminant
            }),
        )
    } else {
        (None, None, None)
    };

    (variant_token, ser_arm, de_arm, storage_arm)
}

/// Process typed variant and generate tokens
fn process_typed_variant(
    variant: &syn::Variant,
    discriminant: usize,
    ts: &TokenStream,
    name_ident: &syn::Ident,
    attributes: &Attributes,
) -> (TokenStream, Option<TokenStream>, Option<TokenStream>, Option<TokenStream>) {
    let ident = variant.ident.to_string();
    let ident_tok = &variant.ident;
    let variant_doc_line = extract_doc_lines(&variant.attrs).join("\n");

    let variant_token = if attributes.wired {
        quote_spanned!(variant.ident.span() => {
            let docs = ::interoptopus::lang::Docs::from_line(#variant_doc_line);
            let ty = ::std::boxed::Box::new(<#ts as ::interoptopus::lang::WireInfo>::wire_info());
            let kind = ::interoptopus::lang::VariantKind::Typed(#discriminant, ty);
            let variant = ::interoptopus::lang::Variant::new(#ident.to_string(), kind, docs);
            variants.push(variant);
        })
    } else {
        quote_spanned!(variant.ident.span() => {
            let docs = ::interoptopus::lang::Docs::from_line(#variant_doc_line);
            let ty = ::std::boxed::Box::new(<#ts as ::interoptopus::lang::TypeInfo>::type_info());
            let kind = ::interoptopus::lang::VariantKind::Typed(#discriminant, ty);
            let variant = ::interoptopus::lang::Variant::new(#ident.to_string(), kind, docs);
            variants.push(variant);
        })
    };

    let (ser_arm, de_arm, storage_arm) = if attributes.wired {
        (
            Some(quote! {
                #name_ident::#ident_tok(data) => {
                    #discriminant.ser(output)?;
                    data.ser(output)?;
                }
            }),
            Some(quote! {
                #discriminant => {
                    let data = ::interoptopus::wire::De::de(input)?;
                    Ok(#name_ident::#ident_tok(data))
                }
            }),
            Some(quote! {
                #name_ident::#ident_tok(data) => 4 + data.storage_size(),
            }),
        )
    } else {
        (None, None, None)
    };

    (variant_token, ser_arm, de_arm, storage_arm)
}

/// Process all enum variants and generate necessary tokens
fn process_enum_variants(item: &ItemEnum, attributes: &Attributes, name_ident: &syn::Ident) -> VariantProcessResult {
    let mut variants = Vec::new();
    let mut ser_arms = Vec::new();
    let mut de_arms = Vec::new();
    let mut storage_arms = Vec::new();
    let mut next_id = 0;

    for variant in &item.variants {
        let discriminant = process_variant_discriminant(variant, &mut next_id);

        let variant_kind = match &variant.fields {
            Fields::Unit => VariantKind::Unit(discriminant),
            Fields::Unnamed(x) => {
                let field_ty = x.unnamed.next().expect("Must have one unnamed field");

                // let nested_type_names = super::nested_types::extract_wire_type_names(&field_ty.first().unwrap().ty);
                // for type_name in nested_type_names {
                //     wire_types.push(quote! {
                //         ::interoptopus::lang::WireType { name: #type_name.to_string() }
                //     });
                // }

                VariantKind::Typed(discriminant, field_ty.to_token_stream())
            }
            Fields::Named(_) => panic!("Named variants are not supported."),
        };

        match variant_kind {
            VariantKind::Unit(x) => {
                let (variant_token, ser_arm, de_arm, storage_arm) = process_unit_variant(variant, x, name_ident, attributes);
                variants.push(variant_token);
                if let Some(arm) = ser_arm {
                    ser_arms.push(arm);
                }
                if let Some(arm) = de_arm {
                    de_arms.push(arm);
                }
                if let Some(arm) = storage_arm {
                    storage_arms.push(arm);
                }
            }
            VariantKind::Typed(x, ts) => {
                let (variant_token, ser_arm, de_arm, storage_arm) = process_typed_variant(variant, x, &ts, name_ident, attributes);
                variants.push(variant_token);
                if let Some(arm) = ser_arm {
                    ser_arms.push(arm);
                }
                if let Some(arm) = de_arm {
                    de_arms.push(arm);
                }
                if let Some(arm) = storage_arm {
                    storage_arms.push(arm);
                }
            }
        }
    }

    VariantProcessResult { variants, ser_arms, de_arms, storage_arms }
}

/// Setup the repr attribute for the enum
fn setup_repr_attribute(item: &mut ItemEnum) {
    let attr_align = quote! { #[repr(u32) ]};

    if item.attrs.iter().any(|attr| attr.path().is_ident("repr")) {
        panic!("Since 0.15 you must not add any `#[repr()] attributes to your enum; Interoptopus will handle that for you.");
    } else {
        item.attrs.push(syn::parse_quote!(#attr_align));
    }
}

pub fn ffi_type_enum(attributes: &Attributes, _input: TokenStream, mut item: ItemEnum) -> TokenStream {
    let doc_line = extract_doc_lines(&item.attrs).join("\n");
    let (type_repr, _) = attributes.type_repr_align();

    let span = item.ident.span();
    let name = item.ident.to_string();
    let ffi_name = attributes.name.clone().unwrap_or_else(|| name.clone());
    let name_ident = syn::Ident::new(&name, span);
    let name_str = syn::LitStr::new(&name, span);
    let namespace = attributes.namespace.clone().unwrap_or_default();

    // Process generic parameters
    let generic_info = process_generic_parameters(&item);

    // Process all enum variants
    let variant_result = process_enum_variants(&item, attributes, &name_ident);

    // Determine layout based on type representation
    let layout = match type_repr {
        TypeRepresentation::C => quote! { ::interoptopus::lang::Layout::C },
        TypeRepresentation::Transparent => quote! { ::interoptopus::lang::Layout::Transparent },
        TypeRepresentation::Primitive(_) => quote! { compile_error!("TODO") },
        _ => quote! { compile_error!("Unsupported repr for enum") },
    };

    // Setup repr attribute
    setup_repr_attribute(&mut item);

    // Extract struct fields for use in quote macro
    let param_param = &generic_info.param_param;
    let param_struct = &generic_info.param_struct;
    let param_where = &generic_info.param_where;
    let variants = &variant_result.variants;

    // Generate type info implementation
    let type_info = if attributes.wired {
        quote! {
            impl #param_param ::interoptopus::lang::WireInfo for #name_ident #param_struct #param_where {
                fn name() -> &'static str { #name_str }

                // For enum, ALL variants must have the same size.
                fn is_fixed_size_element() -> bool { todo!() }

                fn wire_info() -> ::interoptopus::lang::Type {
                    let mut variants = ::std::vec::Vec::new();
                    let docs = ::interoptopus::lang::Docs::from_line(#doc_line);
                    let mut meta = ::interoptopus::lang::Meta::with_module_docs(#namespace.to_string(), docs);

                    #(#variants)*

                    let repr = ::interoptopus::lang::Representation::new(#layout, None);
                    let rval = ::interoptopus::lang::Enum::new(#ffi_name.to_string(), variants, meta, repr);
                    ::interoptopus::lang::Type::WirePayload(::interoptopus::lang::WirePayload::Enum(rval))
                }
            }
        }
    } else {
        quote! {
            unsafe impl #param_param  ::interoptopus::lang::TypeInfo for #name_ident #param_struct #param_where {
                fn type_info() -> ::interoptopus::lang::Type {
                    let mut variants = ::std::vec::Vec::new();
                    let docs = ::interoptopus::lang::Docs::from_line(#doc_line);
                    let mut meta = ::interoptopus::lang::Meta::with_module_docs(#namespace.to_string(), docs);

                    #(#variants)*

                    let repr = ::interoptopus::lang::Representation::new(#layout, None);
                    let rval = ::interoptopus::lang::Enum::new(#ffi_name.to_string(), variants, meta, repr);
                    ::interoptopus::lang::Type::Enum(rval)
                }
            }
        }
    };

    // Generate wire implementation if needed
    let ser_arms = &variant_result.ser_arms;
    let de_arms = &variant_result.de_arms;
    let storage_arms = &variant_result.storage_arms;

    let wires = if attributes.wired {
        quote! {
            impl #param_param ::interoptopus::wire::Ser for #name_ident #param_struct #param_where {
                fn ser(&self, output: &mut impl ::std::io::Write) -> ::std::result::Result<(), ::interoptopus::wire::WireError> {
                    match self {
                        #(#ser_arms)*
                    }
                    Ok(())
                }

                fn storage_size(&self) -> usize {
                    match self {
                        #(#storage_arms)*
                    }
                }
            }

            impl #param_param ::interoptopus::wire::De for #name_ident #param_struct #param_where {
                fn de(input: &mut impl ::std::io::Read) -> ::std::result::Result<Self, ::interoptopus::wire::WireError>
                where
                    Self: Sized
                {
                    let discriminant: usize = ::interoptopus::wire::De::de(input)?;
                    match discriminant {
                        #(#de_arms)*
                        _ => Err(::interoptopus::wire::WireError::InvalidDiscriminant(#name.to_string(), discriminant)),
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #item

        #type_info

        #wires
    }
}
