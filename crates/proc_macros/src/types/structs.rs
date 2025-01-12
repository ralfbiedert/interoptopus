use crate::types::{Attributes, TypeRepresentation};
use crate::util::extract_doc_lines;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
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
#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub fn ffi_type_struct(attributes: &Attributes, _input: TokenStream, mut item: ItemStruct) -> TokenStream {
    let namespace = attributes.namespace.clone().unwrap_or_default();
    let doc_line = extract_doc_lines(&item.attrs).join("\n");

    let (type_repr, align) = attributes.type_repr_align();

    let struct_ident_str = item.ident.to_string();
    let struct_ident = syn::Ident::new(&struct_ident_str, item.ident.span());
    let struct_ident_c = attributes.name.clone().unwrap_or(struct_ident_str);

    let mut field_names = Vec::new();
    let mut field_type_info = Vec::new();
    let mut field_types = Vec::new();
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
                let lt = syn::Lifetime::new(&format!("'{}", ident), item.span());
                generic_parameter_tokens.push(quote! { #lt });
                generic_struct_tokens.push(quote! { #lt });
            }
            GenericParam::Type(ty) => {
                let ident = ty.ident.clone();
                let whre = ty.bounds.to_token_stream();
                generic_parameter_tokens.push(quote! { #ident });
                generic_struct_tokens.push(quote! { #ident });
                generic_params_needing_ctypeinfo_bounds.push(ty.ident.clone());
                generic_where_tokens.push(quote! { #ident: interoptopus::lang::rust::CTypeInfo });
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

                        let first = x.path.segments.first().expect("Must have last path segment.");
                        let middle = x.path.segments.iter().skip(1).rev().skip(1).rev().collect::<Vec<_>>();
                        let last = x.path.segments.last().expect("Must have last path segment.");
                        quote! { < #ty as #first #(:: #middle)*> :: #last }
                    },
                )
            }
            Type::Ptr(x) => x.to_token_stream(),
            Type::Array(x) => x.to_token_stream(),
            Type::Reference(x) => x.to_token_stream(),
            _ => {
                panic!("Unknown token: {field:?}");
            }
        };

        field_type_info.push(quote! { < #token as ::interoptopus::lang::rust::CTypeInfo >::type_info()  });
        field_types.push(quote! { #token });
    }

    let let_fields = if attributes.opaque {
        quote! {}
    } else {
        quote! {
                #({
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(#field_docs);
                    let the_type = #field_type_info;
                    let field = ::interoptopus::lang::c::Field::with_documentation(#field_names.to_string(), the_type, #field_visibilities, documentation);
                    fields.push(field);
                })*
        }
    };

    let let_name = attributes.name.as_ref().map_or_else(
        || {
            quote! {
                #({
                    generics.push(<#generic_params_needing_ctypeinfo_bounds as ::interoptopus::lang::rust::CTypeInfo>::type_info().name_within_lib());
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
        TypeRepresentation::C => quote! { ::interoptopus::lang::c::Layout::C },
        TypeRepresentation::Transparent => quote! { ::interoptopus::lang::c::Layout::Transparent },
        TypeRepresentation::Packed => quote! { ::interoptopus::lang::c::Layout::Packed },
        TypeRepresentation::Opaque => quote! { ::interoptopus::lang::c::Layout::Opaque },
        TypeRepresentation::Primitive(_) => quote! { compile_error!("TODO") },
    };

    let attr_repr = match type_repr {
        TypeRepresentation::C | TypeRepresentation::Opaque => quote! { #[repr(C #attr_align)] },
        TypeRepresentation::Transparent => quote! { #[repr(transparent #attr_align)] },
        TypeRepresentation::Packed => quote! { #[repr(C, packed #attr_align)] },
        TypeRepresentation::Primitive(x) => quote! { #[repr(#x #attr_align)] },
    };

    let rval_builder = if attributes.opaque {
        quote! {
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    } else {
        quote! {
            let repr = ::interoptopus::lang::c::Representation::new(#layout, #align);
            let rval = ::interoptopus::lang::c::CompositeType::with_meta_repr(name, fields, meta, repr);
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    };

    if item.attrs.iter().any(|attr| attr.path().is_ident("repr")) {
        panic!("Since 0.15 you must not add any `#[repr()] attributes to your struct; Interoptopus will handle that for you.");
    } else {
        item.attrs.push(syn::parse_quote!(#attr_repr));
    }

    match type_repr {
        TypeRepresentation::C | TypeRepresentation::Opaque | TypeRepresentation::Packed => {
            quote! {
                #item

                unsafe impl #param_param ::interoptopus::lang::rust::CTypeInfo for #struct_ident #param_struct #param_where {

                    fn type_info() -> ::interoptopus::lang::c::CType {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(#doc_line);
                        let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(#namespace.to_string(), documentation);
                        let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                        let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();

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
                #item

                unsafe impl #param_param ::interoptopus::lang::rust::CTypeInfo for #struct_ident #param_struct #param_where {

                    fn type_info() -> ::interoptopus::lang::c::CType {
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
}
