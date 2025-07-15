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

#[allow(clippy::too_many_lines)]
#[allow(clippy::useless_let_if_seq)]
pub fn ffi_type_enum(attributes: &Attributes, _input: TokenStream, mut item: ItemEnum) -> TokenStream {
    let doc_line = extract_doc_lines(&item.attrs).join("\n");
    let (type_repr, _) = attributes.type_repr_align();

    let span = item.ident.span();
    let name = item.ident.to_string();
    let ffi_name = attributes.name.clone().unwrap_or_else(|| name.clone());
    let name_ident = syn::Ident::new(&name, span);
    let name_str = syn::LitStr::new(&name, span);
    let namespace = attributes.namespace.clone().unwrap_or_default();
    let mut variants = Vec::new();

    let mut next_id = 0;

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

    // Wired Ser/De match arms
    let mut ser_arms = Vec::new();
    let mut de_arms = Vec::new();
    let mut storage_arms = Vec::new();

    for variant in &item.variants {
        let ident = variant.ident.to_string();
        let ident_tok = &variant.ident;
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
                let tokens = quote_spanned!(variant.ident.span() => {
                    let docs = ::interoptopus::lang::Docs::from_line(#variant_doc_line);
                    let kind = ::interoptopus::lang::VariantKind::Unit(#x);
                    let variant = ::interoptopus::lang::Variant::new(#ident.to_string(), kind, docs);
                    variants.push(variant);
                });
                variants.push(tokens);

                if attributes.wired {
                    ser_arms.push(quote! {
                        #name_ident::#ident_tok => {
                            #x.ser(output)?;
                        }
                    });

                    de_arms.push(quote! {
                        #x => Ok(#name_ident::#ident_tok),
                    });

                    storage_arms.push(quote! {
                        #name_ident::#ident_tok => 0usize.storage_size(), // size of discriminant
                    });
                }
            }
            VariantKind::Typed(x, ts) => {
                if !attributes.wired {
                    let tokens = quote_spanned!(variant.ident.span() => {
                        let docs = ::interoptopus::lang::Docs::from_line(#variant_doc_line);
                        let ty = ::std::boxed::Box::new(<#ts as ::interoptopus::lang::TypeInfo>::type_info());
                        let kind = ::interoptopus::lang::VariantKind::Typed(#x, ty);
                        let variant = ::interoptopus::lang::Variant::new(#ident.to_string(), kind, docs);
                        variants.push(variant);
                    });
                    variants.push(tokens);
                } else {
                    let tokens = quote_spanned!(variant.ident.span() => {
                        let docs = ::interoptopus::lang::Docs::from_line(#variant_doc_line);
                        let ty = ::std::boxed::Box::new(<#ts as ::interoptopus::lang::WireInfo>::wire_info());
                        let kind = ::interoptopus::lang::VariantKind::Typed(#x, ty);
                        let variant = ::interoptopus::lang::Variant::new(#ident.to_string(), kind, docs);
                        variants.push(variant);
                    });
                    variants.push(tokens);

                    ser_arms.push(quote! {
                        #name_ident::#ident_tok(data) => {
                            #x.ser(output)?;
                            data.ser(output)?;
                        }
                    });

                    de_arms.push(quote! {
                        #x => {
                            let data = ::interoptopus::lang::De::de(input)?;
                            Ok(#name_ident::#ident_tok(data))
                        }
                    });

                    storage_arms.push(quote! {
                        #name_ident::#ident_tok(data) => 4 + data.storage_size(),
                    });
                }
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

    let mut param_param = quote! {};
    let mut param_struct = quote! {};
    let mut param_where = quote! {};

    if has_generics {
        param_param = quote! { < #(#generic_parameter_tokens),* > };
        param_struct = quote! { < #(#generic_struct_tokens),* > };
        param_where = quote! { where #(#generic_where_tokens),*  };
    }

    let type_info = if attributes.wired {
        quote! {
            impl #param_param ::interoptopus::lang::WireInfo for #name_ident #param_struct #param_where {
                fn name() -> &'static str { #name_str }

                fn wire_info() -> ::interoptopus::lang::Type {
                    let mut variants = ::std::vec::Vec::new();
                    let docs = ::interoptopus::lang::Docs::from_line(#doc_line);
                    let mut meta = ::interoptopus::lang::Meta::with_module_docs(#namespace.to_string(), docs);

                    #({
                        #variants
                    })*

                    let repr = ::interoptopus::lang::Representation::new(#layout, None);
                    let rval = ::interoptopus::lang::Enum::new(#ffi_name.to_string(), variants, meta, repr);
                    ::interoptopus::lang::Type::Domain(::interoptopus::lang::DomainType::Enum(rval))
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

                    #({
                        #variants
                    })*

                    let repr = ::interoptopus::lang::Representation::new(#layout, None);
                    let rval = ::interoptopus::lang::Enum::new(#ffi_name.to_string(), variants, meta, repr);
                    ::interoptopus::lang::Type::Enum(rval)
                }
            }
        }
    };

    let wires = if attributes.wired {
        quote! {
            impl #param_param ::interoptopus::lang::Ser for #name_ident #param_struct #param_where {
                fn ser(&self, output: &mut impl ::std::io::Write) -> ::std::result::Result<(), ::interoptopus::lang::wire::WireError> {
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

            impl #param_param ::interoptopus::lang::De for #name_ident #param_struct #param_where {
                fn de(input: &mut impl ::std::io::Read) -> ::std::result::Result<Self, ::interoptopus::lang::wire::WireError>
                where
                    Self: Sized
                {
                    let discriminant: usize = ::interoptopus::lang::De::de(input)?;
                    match discriminant {
                        #(#de_arms)*
                        _ => Err(::interoptopus::lang::wire::WireError::InvalidDiscriminant(#name.to_string(), discriminant)),
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
