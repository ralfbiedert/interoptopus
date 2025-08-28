use crate::types::TypeRepresentation::Opaque;
use crate::types::{Attributes, TypeRepresentation};
use crate::util::extract_doc_lines;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{GenericParam, ItemStruct, Type};

// Various Struct examples
//
// ```
// pub struct S1<'a, T>
// where
//     T: 'static,
//     T: CTypeInfo,
// {
//     pub x: u32,
//     pub p: PhantomData<&'a T>,
// }
//
// pub struct S2<'a, T: Clone>
// {
//     pub x: u32,
//     pub p: PhantomData<&'a T>,
// }
//
// pub struct Tupled(pub u8);
//
// pub struct S3<const N: usize>
// {
//     pub f: [u32; N],
// }
//
// struct Weird2<'a, T: Clone, const N: usize>
//     where
//         T: Copy + Copy + 'a,
//         T: Debug,
// {
//     t: &'a T,
//     a: [T; N],
//     r: &'a u8,
// }
//
// ```
//
#[allow(clippy::too_many_lines, clippy::cognitive_complexity, clippy::useless_let_if_seq)]
pub fn ffi_type_struct(attributes: &Attributes, _input: TokenStream, mut item: ItemStruct) -> TokenStream {
    let namespace = attributes.namespace.clone().unwrap_or_default();
    let doc_line = extract_doc_lines(&item.attrs).join("\n");

    let (type_repr, align) = attributes.type_repr_align();

    let struct_ident_str = item.ident.to_string();
    let struct_ident_lit = syn::LitStr::new(&struct_ident_str, item.ident.span());
    let struct_ident = syn::Ident::new(&struct_ident_str, item.ident.span());
    let struct_ident_c = attributes.name.clone().unwrap_or(struct_ident_str);

    let mut field_names = Vec::new();
    let mut field_idents = Vec::new();
    let mut field_type_info = Vec::new();
    let mut field_size_info = Vec::new();
    let mut field_types = Vec::new();
    let mut field_wire_de_types = Vec::new();
    let mut field_docs = Vec::new();
    let mut field_visibilities = Vec::new();

    let mut has_generics = false;
    let mut generic_params_needing_ctypeinfo_bounds = Vec::new();
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
                generic_params_needing_ctypeinfo_bounds.push(ty.ident.clone());
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

    if let Some(whre) = &item.generics.where_clause {
        for pred in &whre.predicates {
            generic_where_tokens.push(quote! { #pred });
        }
    }

    let mut param_param = quote! {};
    let mut param_struct = quote! {};
    let mut param_where = quote! {};

    if has_generics {
        param_param = quote! { < #(#generic_parameter_tokens),* > };
        param_struct = quote! { < #(#generic_struct_tokens),* > };
        param_where = quote! { where #(#generic_where_tokens),*  };
    }

    for (i, field) in item.fields.iter().enumerate() {
        let name = field.ident.as_ref().map_or_else(|| format!("x{i}"), ToString::to_string);

        if attributes.skip.contains_key(&name) {
            continue;
        }

        let visibility = attributes.visibility_for_field(field, &name);

        field_names.push(name.clone());
        field_idents.push(field.ident.clone().unwrap_or_else(|| Ident::new(&format!("x{i}"), Span::call_site())));
        field_docs.push(extract_doc_lines(&field.attrs).join("\n"));
        field_visibilities.push(visibility);

        let token = match &field.ty {
            Type::Path(x) => {
                x.qself.as_ref().map_or_else(
                    || x.path.to_token_stream(),
                    |qself| {
                        let ty = &qself.ty;

                        // Ok, I really don't know if this is kosher. After some debugging it seems to work,
                        // but this is probably brittle AF.

                        let first = x.path.segments.first().expect("Must have first path segment.");
                        let middle = x.path.segments.iter().skip(1).rev().skip(1).rev().collect::<Vec<_>>();
                        let last = x.path.segments.last().expect("Must have last path segment.");
                        quote! { < #ty as #first #(:: #middle)*> :: #last }
                    },
                )
            }
            Type::Ptr(x) => x.to_token_stream(),
            Type::Array(x) => x.to_token_stream(),
            Type::Reference(x) => x.to_token_stream(),
            Type::Tuple(x) if type_repr == Opaque => x.to_token_stream(),
            _ => {
                panic!("Field '{name}' has an unsupported type '{:?}'", &field.ty);
            }
        };

        if attributes.wired {
            field_type_info.push(quote! { < #token as ::interoptopus::lang::WireInfo >::wire_info()  });
            field_size_info.push(quote! { < #token as ::interoptopus::lang::WireInfo >::is_fixed_size_element()  });
        } else {
            field_type_info.push(quote! { < #token as ::interoptopus::lang::TypeInfo >::type_info()  });
        }
        field_types.push(quote! { #token });

        // Create turbofish version for deserializer using AST structure
        let wire_de_token = if let Type::Path(type_path) = &field.ty {
            if let Some(qself) = &type_path.qself {
                // Handle qualified self types
                let ty = &qself.ty;
                let first = type_path.path.segments.first().expect("Must have first path segment.");
                let middle = type_path.path.segments.iter().skip(1).rev().skip(1).rev().collect::<Vec<_>>();
                let last = type_path.path.segments.last().expect("Must have last path segment.");
                quote! { < #ty as #first #(:: #middle)*> :: #last }
            } else {
                // Handle regular paths, adding turbofish syntax if generics are present
                let segments = &type_path.path.segments;
                if segments.len() == 1 {
                    let segment = segments.first().unwrap();
                    if segment.arguments.is_empty() {
                        quote! { #token }
                    } else {
                        let ident = &segment.ident;
                        let args = &segment.arguments;
                        quote! { #ident :: #args }
                    }
                } else {
                    // Multi-segment path - only add turbofish to the last segment if it has generics
                    let leading_segments = segments.iter().take(segments.len() - 1);
                    let last_segment = segments.last().unwrap();
                    if last_segment.arguments.is_empty() {
                        quote! { #token }
                    } else {
                        let last_ident = &last_segment.ident;
                        let last_args = &last_segment.arguments;
                        quote! { #(#leading_segments)::* :: #last_ident :: #last_args }
                    }
                }
            }
        } else {
            quote! { #token }
        };
        field_wire_de_types.push(wire_de_token);
    }

    let let_fields = if attributes.opaque {
        quote! {}
    } else {
        quote! {
                #({
                    let docs = ::interoptopus::lang::Docs::from_line(#field_docs);
                    let the_type = #field_type_info;
                    let field = ::interoptopus::lang::Field::with_docs(#field_names.to_string(), the_type, #field_visibilities, docs);
                    fields.push(field);
                })*
        }
    };

    let let_wire_fields = quote! {
        #({
            let docs = ::interoptopus::lang::Docs::from_line(#field_docs);
            let the_type = #field_type_info;
            let field = ::interoptopus::lang::Field::with_docs(#field_names.to_string(), the_type, #field_visibilities, docs);
            wire_fields.push(field);
        })*
    };

    let let_name = attributes.name.as_ref().map_or_else(
        || {
            quote! {
                #({
                    generics.push(<#generic_params_needing_ctypeinfo_bounds as ::interoptopus::lang::TypeInfo>::type_info().name_within_lib());
                })*

                let name = format!("{}{}", #struct_ident_c.to_string(), generics.join(""));
            }
        },
        |name| quote! { let name = #name.to_string(); },
    );

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
        TypeRepresentation::Packed => quote! { ::interoptopus::lang::Layout::Packed },
        TypeRepresentation::Opaque => quote! { ::interoptopus::lang::Layout::Opaque },
        TypeRepresentation::Primitive(_) => quote! { compile_error!("TODO") },
    };

    let attr_repr = if attributes.wired {
        quote! { #[repr(Rust)] }
    } else {
        match type_repr {
            TypeRepresentation::C | TypeRepresentation::Opaque => quote! { #[repr(C #attr_align)] },
            TypeRepresentation::Transparent => quote! { #[repr(transparent #attr_align)] },
            TypeRepresentation::Packed => quote! { #[repr(C, packed #attr_align)] },
            TypeRepresentation::Primitive(x) => quote! { #[repr(#x #attr_align)] },
        }
    };

    let rval_builder = if attributes.opaque {
        quote! {
            let mut rval = ::interoptopus::lang::Opaque::new(name, meta);
            ::interoptopus::lang::Type::Opaque(rval)
        }
    } else {
        quote! {
            let repr = ::interoptopus::lang::Representation::new(#layout, #align);
            let rval = ::interoptopus::lang::Composite::with_meta_repr(name, fields, meta, repr);
            ::interoptopus::lang::Type::Composite(rval)
        }
    };

    if item.attrs.iter().any(|attr| attr.path().is_ident("repr")) {
        panic!("Since 0.15 you must not add any `#[repr()] attributes to your struct; Interoptopus will handle that for you.");
    } else {
        item.attrs.push(syn::parse_quote!(#attr_repr));
    }

    let wires = if attributes.wired {
        quote! {
            impl ::interoptopus::lang::wire::Ser for #struct_ident {
                fn ser(&self, output: &mut impl ::std::io::Write) -> ::std::result::Result<(), ::interoptopus::lang::wire::WireError> {
                    #(
                        self.#field_idents.ser(output)?;
                    )*
                    Ok(())
                }
                fn storage_size(&self) -> usize {
                    0
                    #(
                        + self.#field_idents.storage_size()
                    )*
                }
            }
            impl ::interoptopus::lang::wire::De for #struct_ident {
                fn de(input: &mut impl ::std::io::Read) -> ::std::result::Result<Self, ::interoptopus::lang::wire::WireError>
                where
                    Self: Sized {
                    #(
                        let #field_idents = #field_wire_de_types::de(input)?;
                    )*
                    Ok(Self {
                    #(
                        #field_idents,
                    )*
                    })
                }
            }
        }
    } else {
        quote! {}
    };

    let type_info = if attributes.wired {
        quote! {
            impl #param_param ::interoptopus::lang::WireInfo for #struct_ident #param_struct #param_where {
                fn name() -> &'static str {
                    #struct_ident_lit
                }

                fn is_fixed_size_element() -> bool {
                    true
                        #(
                        && #field_size_info
                        )*
                }

                fn wire_info() -> ::interoptopus::lang::Type {
                    let docs = ::interoptopus::lang::Docs::from_line("");
                    let mut meta = ::interoptopus::lang::Meta::with_module_docs(#namespace.to_string(), docs);
                    let mut wire_fields: ::std::vec::Vec<interoptopus::lang::Field> = ::std::vec::Vec::new();
                    let mut generics: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();

                    #let_name

                    #let_wire_fields

                    let repr = ::interoptopus::lang::Representation::new(#layout, #align);
                    let retval = ::interoptopus::lang::Composite::with_meta_repr(name, wire_fields, meta, repr);
                    ::interoptopus::lang::Type::Domain(::interoptopus::lang::DomainType::Composite(retval))
                }
            }
        }
    } else {
        match type_repr {
            TypeRepresentation::C | TypeRepresentation::Opaque | TypeRepresentation::Packed => {
                quote! {
                    unsafe impl #param_param ::interoptopus::lang::TypeInfo for #struct_ident #param_struct #param_where {

                        fn type_info() -> ::interoptopus::lang::Type {
                            let docs = ::interoptopus::lang::Docs::from_line(#doc_line);
                            let mut meta = ::interoptopus::lang::Meta::with_module_docs(#namespace.to_string(), docs);
                            let mut fields: ::std::vec::Vec<interoptopus::lang::Field> = ::std::vec::Vec::new();
                            let mut generics: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();

                            #let_name

                            #let_fields

                            #rval_builder
                        }
                    }
                }
            }
            TypeRepresentation::Transparent => {
                let first_field_type = field_types.first().expect("Transparent structs must have at least one field");

                quote! {
                    unsafe impl #param_param ::interoptopus::lang::TypeInfo for #struct_ident #param_struct #param_where {

                        fn type_info() -> ::interoptopus::lang::Type {
                            < #first_field_type > :: type_info()
                        }
                    }
                }
            }
            TypeRepresentation::Primitive(_) => {
                quote! {
                    compile_error!("Attributes u8, ..., u64 are only allowed on enums.");
                }
            }
        }
    };

    quote! {
        #item

        #type_info

        #wires
    }
}
